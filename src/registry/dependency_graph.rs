use std::iter::FromIterator;
use std::cmp::{max, Ordering};
use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};
use queues::{IsQueue, Queue};
use crate::service_types::article_code::ArticleCode;
use crate::service_types::article_define::{ArticleDefine, IArticleDefine};
use crate::registry_providers::article_provider::{BoxArticleSpec, BoxArticleSpecList};
use crate::registry_providers::concept_provider::{BoxConceptSpec};

#[derive(Debug, Copy, Clone)]
struct ArticleEdge {
    start: ArticleCode,
    stops: ArticleCode,
}

impl Hash for ArticleEdge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.value.hash(state);
        self.stops.value.hash(state);
    }
}

impl PartialEq<Self> for ArticleEdge {
    fn eq(&self, other: &Self) -> bool {
        if self.start.get_value() != other.start.get_value() {
            return false;
        }
        if self.stops.get_value() == other.stops.get_value() {
            return false;
        }
        true
    }
}

impl Eq for ArticleEdge {
}

impl ArticleEdge {
    pub(crate) fn new(_start: &ArticleCode, _stops: &ArticleCode) -> ArticleEdge {
        ArticleEdge {
            start: ArticleCode::from_code(_start),
            stops: ArticleCode::from_code(_stops),
        }
    }
}

pub(crate) struct DependencyGraph {
}

impl DependencyGraph {
    fn article_code_order_fun(x: &ArticleCode, y: &ArticleCode) -> Ordering {
        if x.get_value() < y.get_value() {
            return Ordering::Less;
        }
        if x.get_value() > y.get_value() {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
    fn concept_code_finder_fun(article: &BoxArticleSpec) -> impl FnMut(&&BoxConceptSpec) -> bool + '_ {
        move |x: &&BoxConceptSpec| -> bool {
            x.get_code().get_value() == article.get_role().get_value()
        }
    }
    fn article_edge_order_fun(x: &ArticleEdge, y: &ArticleEdge) -> Ordering {
        if x.start.get_value() == y.start.get_value() {
            if x.stops.get_value() < y.stops.get_value() {
                return Ordering::Less;
            }
            if x.stops.get_value() > y.stops.get_value() {
                return Ordering::Greater;
            }
            return Ordering::Equal;
        }
        if x.start.get_value() < y.start.get_value() {
            return Ordering::Less;
        }
        if x.start.value > y.start.value {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
    fn reduce_edges_func(concept_model: &Vec<BoxConceptSpec>) -> impl FnMut(HashSet<ArticleEdge>, &BoxArticleSpec) -> HashSet<ArticleEdge> + '_ {
        move |agr: HashSet<ArticleEdge>, x: &BoxArticleSpec| -> HashSet<ArticleEdge> {
            DependencyGraph::merge_edges(concept_model, &agr, x)
        }
    }
    fn define_compare(order: &Vec<ArticleCode>) -> impl FnMut(&ArticleDefine, &ArticleDefine) -> Ordering + '_ {
        move |x: &ArticleDefine, y: &ArticleDefine| -> Ordering {
            let x_index = order.iter().position(DependencyGraph::article_define_finder_fun(&x));

            let y_index = order.iter().position(DependencyGraph::article_define_finder_fun(&y));

            DependencyGraph::compare_to(x_index, y_index)
        }
    }
    pub(crate) fn init_graph_model(article_model: &Vec<BoxArticleSpec>, concept_model: &Vec<BoxConceptSpec>) -> (Vec<ArticleCode>, HashMap<ArticleCode, Vec<ArticleDefine>>) {
        let vert_model: Vec<ArticleCode> = DependencyGraph::create_vert_model(article_model);
        let edge_model: Vec<ArticleEdge> = DependencyGraph::create_edge_model(article_model, concept_model);
        let pend_model: Vec<ArticleEdge> = DependencyGraph::create_pend_model(article_model, concept_model);

        let order = DependencyGraph::create_topo_model(&vert_model, &edge_model);
        let paths = DependencyGraph::create_path_model(&article_model, &vert_model, &pend_model, &order);

        (order, paths)
    }
    fn create_vert_model(article_model: &Vec<BoxArticleSpec>) -> Vec<ArticleCode> {
        let mut vert_model: Vec<ArticleCode> = article_model.iter().map(|a| a.get_code()).collect();
        vert_model.sort_by(DependencyGraph::article_code_order_fun);
        vert_model
    }
    fn create_edge_model(article_model: &Vec<BoxArticleSpec>, concept_model: &Vec<BoxConceptSpec>) -> Vec<ArticleEdge> {
        let init: HashSet<ArticleEdge> = HashSet::new();

        let edge_model = article_model.iter().fold(init, DependencyGraph::reduce_edges_func(concept_model));

        let mut sort_model = Vec::from_iter(edge_model.into_iter());

        sort_model.sort_by(DependencyGraph::article_edge_order_fun);
        sort_model
    }
    fn merge_edges(concept_model: &Vec<BoxConceptSpec>, edge_model: &HashSet<ArticleEdge>, article: &BoxArticleSpec) -> HashSet<ArticleEdge> {
        let mut result: HashSet<ArticleEdge> = HashSet::clone(edge_model);

        let option_concept = concept_model.iter().find(DependencyGraph::concept_code_finder_fun(&article));

        if option_concept.is_some() {
            let concept = option_concept.unwrap();
            article.get_sums().iter().for_each(|s| {
                result.insert(ArticleEdge::new(&article.get_code(), &s));
            });
            concept.get_path().iter().for_each(|p| {
                result.insert(ArticleEdge::new(&p, &article.get_code()));
            });
        }
        result
    }
    fn create_pend_model(article_model: &Vec<BoxArticleSpec>, concept_model: &Vec<BoxConceptSpec>) -> Vec<ArticleEdge> {
        let reduce_func = |agr: HashSet<ArticleEdge>, x: &BoxArticleSpec| -> HashSet<ArticleEdge> {
            DependencyGraph::merge_pends(concept_model, agr, x)
        };
        let init: HashSet<ArticleEdge> = HashSet::new();

        let pend_model = article_model.iter().fold(init, reduce_func);
        let mut sort_model = Vec::from_iter(pend_model.into_iter());

        sort_model.sort_by(DependencyGraph::article_edge_order_fun);
        sort_model
    }
    fn merge_pends(concept_model: &Vec<BoxConceptSpec>, edge_model: HashSet<ArticleEdge>, article: &BoxArticleSpec) -> HashSet<ArticleEdge> {
        let mut result = HashSet::from(edge_model);

        let option_concept = concept_model.iter().find(DependencyGraph::concept_code_finder_fun(article));

        if option_concept.is_some() {
            let concept = option_concept.unwrap();
             concept.get_path().iter().for_each(|p| {
                result.insert(ArticleEdge::new(&p, &article.get_code()));
            });
        }
        result
    }
    fn create_topo_model(vert_model: &Vec<ArticleCode>, edge_model: &Vec<ArticleEdge>) -> Vec<ArticleCode> {
        let mut degrees: HashMap<i32, i32> = vert_model.iter()
            .map(|x| {
                (x.get_value(), edge_model.iter().fold(0, |agr, e| -> i32 {
                    if e.stops.get_value() == x.get_value() { agr+1 } else { agr }
                }))
            }).collect();

        let mut articles_order = Vec::new();

        let mut queue_entries: Vec<i32> = degrees.iter()
            .filter(|x| *x.1 == 0)
            .map(|x| *x.0).collect();

        queue_entries.sort();

        let mut queues: Queue<i32> = Queue::new();
        queue_entries.into_iter().for_each(|x| {
            let _res = queues.add(x);
        });
        let mut index = 0;
        while queues.size() != 0 {
            index = index + 1;
            let result_article = queues.remove();
            if result_article.is_ok() {
                let article = result_article.unwrap();
                articles_order.push(ArticleCode::get(article));

                let paths: Vec<ArticleCode> = edge_model.iter().filter(|x| {
                    x.start.get_value() == article
                }).map(|x| ArticleCode::from_code(&x.stops)).collect();

                paths.into_iter().for_each(|p| {
                    degrees.entry(p.get_value()).and_modify(|e| *e = max(0, *e-1));

                    if degrees[&p.get_value()] == 0 {
                        let _res = queues.add(p.get_value());
                    }
                });
            }
        }
        if index != vert_model.len() {
            return Vec::new();
        }
        articles_order
    }
    fn create_path_model(article_model: &BoxArticleSpecList, vert_model: &Vec<ArticleCode>, pend_model: &Vec<ArticleEdge>, vert_order: &Vec<ArticleCode>) -> HashMap<ArticleCode, Vec<ArticleDefine>> {
        vert_model.into_iter().map(|x| {
            (ArticleCode::from_code(x), DependencyGraph::merge_paths(article_model, pend_model, vert_order, x))
        }).collect()
    }
    fn merge_paths(article_model: &Vec<BoxArticleSpec>, edge_model: &Vec<ArticleEdge>, vert_order: &Vec<ArticleCode>, article: &ArticleCode) -> Vec<ArticleDefine> {
        let article_iter: Vec<ArticleDefine> = edge_model.iter().filter(|e| e.stops.get_value() == article.get_value()).map(|e| {
            DependencyGraph::get_article_defs(&e.start, article_model)
        }).collect();
        let article_init: Vec<ArticleDefine> = article_iter.iter().map(|x| ArticleDefine::from_defs(x)).collect();
        let article_path = article_iter.iter().fold(article_init, |agr, x| {
            DependencyGraph::merge_vert(article_model, edge_model, agr, x)
        });
        DependencyGraph::article_defs_distinct_sort(&article_path, vert_order)
    }
    fn merge_vert(articles_model: &BoxArticleSpecList, edge_model: &Vec<ArticleEdge>, defs_model: Vec<ArticleDefine>, defs: &ArticleDefine) -> Vec<ArticleDefine> {
       let result_iter: Vec<ArticleDefine> = edge_model.iter().filter(|e| e.stops.get_value() == defs.get_code().get_value()).
            map(|e| {
                DependencyGraph::get_article_defs(&e.start, articles_model)
            }).collect();
        let result_init: Vec<ArticleDefine> = result_iter.iter().map(|x| ArticleDefine::from_defs(x)).collect();
        let result_list: Vec<ArticleDefine> = result_iter.iter().fold(result_init, |agr, item| {
            DependencyGraph::merge_vert(articles_model, edge_model, agr, item)
        });
        return vec![defs_model, result_list, vec![ArticleDefine::from_defs(defs)]].concat();
    }
    fn article_defs_distinct_sort(array: &Vec<ArticleDefine>, order: &Vec<ArticleCode>) -> Vec<ArticleDefine> {
        let mut distinct: Vec<ArticleDefine> = vec![];
        array.iter().for_each(|x| {
            let found = distinct.iter().any(|ax| {
                ax.get_code().get_value() == x.get_code().get_value()
            });
            if found == false {
                distinct.push(ArticleDefine::from_defs(x));
            }
        });
        distinct.sort_by(DependencyGraph::define_compare(order));
        distinct
    }
    fn get_article_defs(article: &ArticleCode, articles_model: &BoxArticleSpecList) -> ArticleDefine {
        let option_article_spec = articles_model.iter().find(|x| x.get_code().get_value()==article.get_value());
        if option_article_spec.is_some() {
            let article_spec = option_article_spec.unwrap();
            return article_spec.get_defs();
        }
        return ArticleDefine::new();
    }
    fn article_define_finder_fun(defs: &ArticleDefine) -> impl FnMut(&ArticleCode) -> bool + '_ {
        move |x| {
            x.get_value() == defs.get_code().get_value()
        }
    }
    fn compare_to(x_index: Option<usize>, y_index: Option<usize>) -> Ordering {
        if x_index.is_none() && y_index.is_none() {
            return Ordering::Equal;
        }
        if x_index.is_none() && y_index.is_some() {
            return Ordering::Less;
        }
        if x_index.is_some() && y_index.is_none() {
            return Ordering::Greater;
        }
        if x_index.unwrap() > y_index.unwrap() {
            return Ordering::Greater;
        }
        if x_index.unwrap() < y_index.unwrap() {
            return Ordering::Less;
        }
        return Ordering::Equal
    }
}