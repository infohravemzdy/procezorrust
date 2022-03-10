use std::iter::FromIterator;
use std::cmp::{max, Ordering};
use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};
use queues::{IsQueue, Queue};
use crate::service_types::article_code::ArticleCode;
use crate::service_types::article_define::{ArticleDefine, IArticleDefine};
use crate::registry_providers::article_provider::{ArcArticleSpec, ArcArticleSpecList};
use crate::registry_providers::concept_provider::{BoxConceptSpec};
use crate::service_types::article_term::ArticleTerm;

#[derive(Debug, Copy, Clone)]
struct ArticleEdge {
    start: ArticleTerm,
    stops: ArticleTerm,
}

impl Hash for ArticleEdge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.stops.hash(state);
    }
}

impl PartialEq<Self> for ArticleEdge {
    fn eq(&self, other: &Self) -> bool {
        if self.start.get_code().get_value() != other.start.get_code().get_value() {
            return false;
        }
        if self.stops.get_code().get_value() == other.stops.get_code().get_value() {
            return false;
        }
        true
    }
}

impl Eq for ArticleEdge {
}

impl ArticleEdge {
    pub(crate) fn new(_start: &ArticleTerm, _stops: &ArticleTerm) -> ArticleEdge {
        ArticleEdge {
            start: ArticleTerm::from_term(_start),
            stops: ArticleTerm::from_term(_stops),
        }
    }
}

pub(crate) struct DependencyGraph {
}

impl DependencyGraph {
    fn article_term_order_fun(x: &ArticleTerm, y: &ArticleTerm) -> Ordering {
        if x.get_seqs_value() < y.get_seqs_value() {
            return Ordering::Less;
        }
        if x.get_seqs_value() > y.get_seqs_value() {
            return Ordering::Greater;
        }
        if x.get_code_value() < y.get_code_value() {
            return Ordering::Less;
        }
        if x.get_code_value() > y.get_code_value() {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
    fn concept_code_finder_fun(article: &ArcArticleSpec) -> impl FnMut(&&BoxConceptSpec) -> bool + '_ {
        move |x: &&BoxConceptSpec| -> bool {
            x.get_code().get_value() == article.get_role().get_value()
        }
    }
    fn article_edge_order_fun(x: &ArticleEdge, y: &ArticleEdge) -> Ordering {
        if DependencyGraph::article_term_order_fun(&x.start, &y.start) == Ordering::Equal {
            return DependencyGraph::article_term_order_fun(&x.stops, &y.stops);
        }
        return DependencyGraph::article_term_order_fun(&x.start, &y.start);
    }
    fn reduce_edges_func<'a>(article_model: &'a Vec<ArcArticleSpec>, concept_model: &'a Vec<BoxConceptSpec>) -> impl Fn(HashSet<ArticleEdge>, &ArcArticleSpec) -> HashSet<ArticleEdge> + 'a  {
        move |agr: HashSet<ArticleEdge>, x: &ArcArticleSpec| -> HashSet<ArticleEdge> {
            DependencyGraph::merge_edges(article_model, concept_model, agr, x)
        }
    }
    fn define_compare(term_order: &Vec<ArticleTerm>) -> impl FnMut(&ArticleDefine, &ArticleDefine) -> Ordering + '_ {
        move |x: &ArticleDefine, y: &ArticleDefine| -> Ordering {
            let x_index = term_order.iter().position(DependencyGraph::article_define_finder_fun(&x));

            let y_index = term_order.iter().position(DependencyGraph::article_define_finder_fun(&y));

            DependencyGraph::compare_to(x_index, y_index)
        }
    }
    pub(crate) fn init_graph_model(article_model: &Vec<ArcArticleSpec>, concept_model: &Vec<BoxConceptSpec>) -> (Vec<ArticleTerm>, HashMap<ArticleTerm, Vec<ArticleDefine>>) {
        let vert_model: Vec<ArticleTerm> = DependencyGraph::create_vert_model(article_model);
        let edge_model: Vec<ArticleEdge> = DependencyGraph::create_edge_model(article_model, concept_model);
        let pend_model: Vec<ArticleEdge> = DependencyGraph::create_pend_model(article_model, concept_model);

        let order = DependencyGraph::create_topo_model(&vert_model, &edge_model);
        let paths = DependencyGraph::create_path_model(&article_model, &vert_model, &pend_model, &order);

        (order, paths)
    }
    fn create_vert_model(article_model: &Vec<ArcArticleSpec>) -> Vec<ArticleTerm> {
        let mut vert_model: Vec<ArticleTerm> = article_model.iter().map(|a| a.get_term()).collect();
        vert_model.sort_by(DependencyGraph::article_term_order_fun);
        vert_model
    }
    fn create_edge_model(article_model: &Vec<ArcArticleSpec>, concept_model: &Vec<BoxConceptSpec>) -> Vec<ArticleEdge> {
        let init: HashSet<ArticleEdge> = HashSet::new();

        let edge_model = article_model.iter().fold(init, DependencyGraph::reduce_edges_func(article_model, concept_model));

        let mut sort_model = Vec::from_iter(edge_model.into_iter());

        sort_model.sort_by(DependencyGraph::article_edge_order_fun);
        sort_model
    }
    fn merge_edges(articles_model: &ArcArticleSpecList, concept_model: &Vec<BoxConceptSpec>, edge_model: HashSet<ArticleEdge>, article: &ArcArticleSpec) -> HashSet<ArticleEdge> {
        let mut result: HashSet<ArticleEdge> = HashSet::clone(&edge_model);

        let option_concept = concept_model.iter().find(DependencyGraph::concept_code_finder_fun(&article));

        if option_concept.is_some() {
            let concept = option_concept.unwrap();
            article.get_sums().iter().for_each(|s| {
                result.insert(ArticleEdge::new(&article.get_term(), &DependencyGraph::get_article_term(&s, articles_model)));
            });
            concept.get_path().iter().for_each(|p| {
                result.insert(ArticleEdge::new(&DependencyGraph::get_article_term(&p, articles_model), &article.get_term()));
            });
        }
        result
    }
    fn create_pend_model(article_model: &Vec<ArcArticleSpec>, concept_model: &Vec<BoxConceptSpec>) -> Vec<ArticleEdge> {
        let reduce_func = |agr: HashSet<ArticleEdge>, x: &ArcArticleSpec| -> HashSet<ArticleEdge> {
            DependencyGraph::merge_pends(article_model, concept_model, agr, x)
        };
        let init: HashSet<ArticleEdge> = HashSet::new();

        let pend_model = article_model.iter().fold(init, reduce_func);
        let mut sort_model = Vec::from_iter(pend_model.into_iter());

        sort_model.sort_by(DependencyGraph::article_edge_order_fun);
        sort_model
    }
    fn merge_pends(article_model: &Vec<ArcArticleSpec>, concept_model: &Vec<BoxConceptSpec>, edge_model: HashSet<ArticleEdge>, article: &ArcArticleSpec) -> HashSet<ArticleEdge> {
        let mut result = HashSet::from(edge_model);

        let option_concept = concept_model.iter().find(DependencyGraph::concept_code_finder_fun(article));

        if option_concept.is_some() {
            let concept = option_concept.unwrap();
             concept.get_path().iter().for_each(|p| {
                result.insert(ArticleEdge::new(&DependencyGraph::get_article_term(&p, article_model), &article.get_term()));
            });
        }
        result
    }
    fn create_topo_model(vert_model: &Vec<ArticleTerm>, edge_model: &Vec<ArticleEdge>) -> Vec<ArticleTerm> {
        let mut degrees: HashMap<(i32, i16), i32> = vert_model.iter()
            .map(|x| {
                ((x.get_code_value(), x.get_seqs_value()), edge_model.iter().fold(0, |agr, e| -> i32 {
                    if e.stops.get_code_value() == x.get_code_value() { agr+1 } else { agr }
                }))
            }).collect();

        let mut articles_order = Vec::new();

        let mut queue_entries: Vec<ArticleTerm> = degrees.iter()
            .filter(|((_c,_s), d)| **d == 0)
            .map(|((c,s), _d)| ArticleTerm::get(*c, *s)).collect();

        queue_entries.sort_by(DependencyGraph::article_term_order_fun);

        let mut queues: Queue<(i32, i16)> = Queue::new();
        queue_entries.into_iter().for_each(|x| {
            let _res = queues.add((x.get_code_value(),x.get_seqs_value()));
        });
        let mut index = 0;
        while queues.size() != 0 {
            index = index + 1;
            let result_article = queues.remove();
            if result_article.is_ok() {
                let article = result_article.unwrap();
                articles_order.push(ArticleTerm::get(article.0, article.1));

                let paths: Vec<ArticleTerm> = edge_model.iter().filter(|x| {
                    x.start.get_code_value() == article.0
                }).map(|x| ArticleTerm::from_term(&x.stops)).collect();

                paths.into_iter().for_each(|p| {
                    degrees.entry((p.get_code_value(), p.get_seqs_value())).and_modify(|e| *e = max(0, *e-1));

                    if degrees[&(p.get_code_value(), p.get_seqs_value())] == 0 {
                        let _res = queues.add((p.get_code_value(), p.get_seqs_value()));
                    }
                });
            }
        }
        let model_length = vert_model.len();
        if index != vert_model.len() {
            println!("CreateTopoModel, build graph failed: {}<>{}", index, model_length);
            return Vec::new();
        }
        articles_order
    }
    fn create_path_model(article_model: &ArcArticleSpecList, vert_model: &Vec<ArticleTerm>, pend_model: &Vec<ArticleEdge>, vert_order: &Vec<ArticleTerm>) -> HashMap<ArticleTerm, Vec<ArticleDefine>> {
        vert_model.into_iter().map(|x| {
            (ArticleTerm::from_term(x), DependencyGraph::merge_paths(article_model, pend_model, vert_order, x))
        }).collect()
    }
    fn merge_paths(article_model: &Vec<ArcArticleSpec>, edge_model: &Vec<ArticleEdge>, vert_order: &Vec<ArticleTerm>, article: &ArticleTerm) -> Vec<ArticleDefine> {
        let article_iter: Vec<ArticleDefine> = edge_model.iter().filter(|e| e.stops.get_code_value() == article.get_code_value()).map(|e| {
            DependencyGraph::get_article_defs(&e.start.code, article_model)
        }).collect();
        let article_init: Vec<ArticleDefine> = article_iter.iter().map(|x| ArticleDefine::from_defs(x)).collect();
        let article_path = article_iter.iter().fold(article_init, |agr, x| {
            DependencyGraph::merge_vert(article_model, edge_model, agr, x)
        });
        DependencyGraph::article_defs_distinct_sort(&article_path, vert_order)
    }
    fn merge_vert(articles_model: &ArcArticleSpecList, edge_model: &Vec<ArticleEdge>, defs_model: Vec<ArticleDefine>, defs: &ArticleDefine) -> Vec<ArticleDefine> {
       let result_iter: Vec<ArticleDefine> = edge_model.iter().filter(|e| e.stops.get_code_value() == defs.get_code().get_value()).
            map(|e| {
                DependencyGraph::get_article_defs(&e.start.code, articles_model)
            }).collect();
        let result_init: Vec<ArticleDefine> = result_iter.iter().map(|x| ArticleDefine::from_defs(x)).collect();
        let result_list: Vec<ArticleDefine> = result_iter.iter().fold(result_init, |agr, item| {
            DependencyGraph::merge_vert(articles_model, edge_model, agr, item)
        });
        return vec![defs_model, result_list, vec![ArticleDefine::from_defs(defs)]].concat();
    }
    fn article_defs_distinct_sort(array: &Vec<ArticleDefine>, order: &Vec<ArticleTerm>) -> Vec<ArticleDefine> {
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
    fn get_article_term(article: &ArticleCode, articles_model: &ArcArticleSpecList) -> ArticleTerm {
        let option_article_spec = articles_model.iter().find(|x| x.get_code().get_value()==article.get_value());
        if option_article_spec.is_some() {
            let article_spec = option_article_spec.unwrap();
            return article_spec.get_term();
        }
        return ArticleTerm::new();
    }
    fn get_article_defs(article: &ArticleCode, articles_model: &ArcArticleSpecList) -> ArticleDefine {
        let option_article_spec = articles_model.iter().find(|x| x.get_code().get_value()==article.get_value());
        if option_article_spec.is_some() {
            let article_spec = option_article_spec.unwrap();
            return article_spec.get_defs();
        }
        return ArticleDefine::new();
    }
    fn article_define_finder_fun(defs: &ArticleDefine) -> impl FnMut(&ArticleTerm) -> bool + '_ {
        move |x| {
            x.get_code_value() == defs.get_code().get_value()
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