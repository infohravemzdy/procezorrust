use std::sync::Arc;
use std::cmp::Ordering;
use std::collections::HashMap;
use legalios::service::period::{IPeriod, Period};
use legalios::service::bundle_props::IBundleProps;
use crate::registry::dependency_graph::DependencyGraph;
use crate::registry_factories::article_factory::IArticleSpecFactory;
use crate::registry_factories::concept_factory::IConceptSpecFactory;
use crate::registry_providers::article_provider::{BoxArticleSpec};
use crate::registry_providers::concept_provider::{BoxConceptSpec, ResultFunc};
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::version_code::VersionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::term_result::{ResultArcTermResultList};
use crate::service_types::term_target::{ArcTermTarget, ArcTermTargetList, TermTarget};
use crate::service_types::article_define::{ArticleDefine, IArticleDefine};
use crate::registry::term_calcul::{BoxTermCalcul, BoxTermCalculList, TermCalcul};
use crate::service_errors::term_result_error::TermResultError;
use crate::service_types::contract_term::ArcContractTermList;
use crate::service_types::position_term::ArcPositionTermList;

pub(crate) trait IResultBuilder {
    fn is_valid(&self) -> bool;
    fn get_version(&self) -> &VersionCode;
    fn get_period_init(&self) -> &dyn IPeriod;
    fn init_with_period(&mut self,
                        version: &VersionCode,
                        period: &dyn IPeriod,
                        article_factory: &Box<dyn IArticleSpecFactory>,
                        concept_factory: &Box<dyn IConceptSpecFactory>) -> bool;
    fn order(&self) -> &Vec<ArticleCode>;
    fn paths(&self) -> &HashMap<ArticleCode, Vec<ArticleDefine>>;
    fn get_results(&self, ruleset: &dyn IBundleProps,
                   contracts: &ArcContractTermList, positions: &ArcPositionTermList,
                   targets: &ArcTermTargetList, calc_arts: &Vec<ArticleCode>) -> ResultArcTermResultList;
}

pub(crate) struct ResultBuilder {
    version: VersionCode,
    period_init: Period,
    article_order: Vec<ArticleCode>,
    article_paths: HashMap<ArticleCode, Vec<ArticleDefine>>,
    article_model: Vec<BoxArticleSpec>,
    concept_model: Vec<BoxConceptSpec>,
}

#[allow(dead_code)]
impl ResultBuilder {
    pub(crate) fn new() -> ResultBuilder {
        ResultBuilder{
            version: VersionCode::new(),
            period_init: Period::new(),
            article_order: vec![],
            article_paths: HashMap::new(),
            article_model: vec![],
            concept_model: vec![],
        }
    }
}

impl ResultBuilder {
    fn build_calculs_list(&self, period: &dyn IPeriod, ruleset: &dyn IBundleProps,
                          contract_terms: &ArcContractTermList, position_terms: &ArcPositionTermList,
                          targets: &ArcTermTargetList, calc_arts: &Vec<ArticleCode>) -> BoxTermCalculList {
        let spec_defines: Vec<Option<&BoxArticleSpec>> = calc_arts.iter().map(|a| self.article_model.iter()
            .find(|m| m.get_code() == *a)).collect();

        let calc_defines: Vec<ArticleDefine> = spec_defines.iter()
            .filter(|s| s.is_some())
            .map(|t| t.unwrap())
            .map(|x| ArticleDefine::get(x.get_code().value, x.get_seqs().value, x.get_role().value)).collect();

        let targets_init = targets.to_vec();

        let targets_spec = self.add_fin_def_to_targets(period, ruleset,
                                                       contract_terms, position_terms,
                                                       targets_init, &calc_defines);

        let targets_step = self.add_extern_to_targets(period, ruleset,
                                                      contract_terms, position_terms, targets_spec);

        let calculs_list = self.add_target_to_calculs(targets_step);

        calculs_list
    }
    fn build_results_list(period: &dyn IPeriod, ruleset: &dyn IBundleProps, calculs: BoxTermCalculList) -> ResultArcTermResultList {
        let reduce_func = |agr: ResultArcTermResultList, x: &BoxTermCalcul| -> ResultArcTermResultList {
            let results = x.get_results(period, ruleset, &agr);
            ResultBuilder::merge_results(agr, results)
        };
        let results_init: ResultArcTermResultList = vec![];
        calculs.iter().fold(results_init, reduce_func)
    }

    fn add_fin_def_to_targets(&self, period: &dyn IPeriod, ruleset: &dyn IBundleProps,
                              contract_terms: &ArcContractTermList, position_terms: &ArcPositionTermList,
                              targets: ArcTermTargetList, calc_defs: &Vec<ArticleDefine>) -> ArcTermTargetList {
        self.merge_list_pendings(period, ruleset, contract_terms, position_terms, targets, calc_defs)
    }

    fn add_extern_to_targets(&self, period: &dyn IPeriod, ruleset: &dyn IBundleProps,
                             contract_terms: &ArcContractTermList, position_terms: &ArcPositionTermList,
                             targets: ArcTermTargetList) -> ArcTermTargetList {
        let reduce_func = |agr: ArcTermTargetList, item: &ArcTermTarget| -> ArcTermTargetList {
            self.merge_pendings(period, ruleset,
                                contract_terms, position_terms,
                                agr, item)
        };

        let targets_init = targets.to_vec();

        let mut target_list = targets.iter().fold(targets_init, reduce_func);

        target_list.sort_by( |x, y| -> Ordering {
            let x_index = self.article_order.iter().position(ResultBuilder::target_finder_fun(x));

            let y_index = self.article_order.iter().position(ResultBuilder::target_finder_fun(y));

            ResultBuilder::compare_to(x_index, y_index)
        });

        target_list
    }

    fn add_defines_to_targets(&self, period: &dyn IPeriod, ruleset: &dyn IBundleProps,
                              contract_terms: &ArcContractTermList, position_terms: &ArcPositionTermList,
                              targets: ArcTermTargetList, defines: &Vec<ArticleDefine>) -> ArcTermTargetList {
        return defines.iter()
            .flat_map(|x| {
                let init_targets: ArcTermTargetList = targets.iter()
                    .filter(|t| t.get_article() == x.get_code()).map(|x| *x).collect();
                return ResultBuilder::get_target_list(period, ruleset, &self.concept_model,
                                                      contract_terms, position_terms,
                                                      &init_targets, &x.get_code(), &x.get_role());

            }).collect();
    }

    fn add_target_to_calculs(&self, targets: ArcTermTargetList) -> BoxTermCalculList {
        let targets_rets = targets.iter()
            .map(|x| {
                let articleSpec = self.article_model.into_iter().find(
                    |a| a.get_code() == x.get_article());
                Box::new(
                    TermCalcul::new(x, articleSpec, Self::get_calcul_func(&self.concept_model, &x.get_concept()))
                ) as BoxTermCalcul
            }).collect();
         targets_rets
    }

    fn merge_targets(init: ArcTermTargetList, results: ArcTermTargetList) -> ArcTermTargetList {
        vec![init, results].concat()
    }
    fn merge_results(init: ResultArcTermResultList, results: ResultArcTermResultList) -> ResultArcTermResultList {
        vec![init, results].concat()
    }
    fn get_target_list(period: &dyn IPeriod, ruleset: &dyn IBundleProps, concepts_model: &Vec<BoxConceptSpec>,
                     contract_terms: &ArcContractTermList, position_terms: &ArcPositionTermList,
                     targets: &ArcTermTargetList, article: &ArticleCode, concept: &ConceptCode) -> ArcTermTargetList {
        let month_code = MonthCode::get(period.get_code());
        let variant = VariantCode::get(1);

        let concept_spec = concepts_model.iter().find(|a| a.get_code() == *concept);
        if concept_spec.is_some() {
            let contract = ContractCode::new();
            let position = PositionCode::new();
            return vec![Arc::new(TermTarget::new(&month_code, &contract, &position, &variant, article, concept))];
        }
        return concept_spec.unwrap()
            .default_target_list(article, period, ruleset, &month_code,
                                                contract_terms, position_terms, targets, variant);
    }
    fn get_calcul_func(concepts_model: &Vec<BoxConceptSpec>, concept: &ConceptCode) -> Option<ResultFunc> {
        let concept_spec = concepts_model.iter().find(|x| x.get_code().get_value()==concept.get_value());
        if concept_spec.is_none() {
            return Some(Self::not_found_calcul_func);
        }
        concept_spec.unwrap().get_result_delegate()
    }

    fn not_found_calcul_func(target: ArcTermTarget, spec: Option<BoxArticleSpec>,
                             period: &dyn IPeriod, _ruleset: &dyn IBundleProps,
                             _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let result_error = TermResultError::no_result_func_error(period, &target);
        vec![Err(result_error)]
    }
    fn merge_pendings(&self, period: &dyn IPeriod, ruleset: &dyn IBundleProps,
                      contract_terms: &ArcContractTermList, position_terms: &ArcPositionTermList,
                      targets: ArcTermTargetList, target: &ArcTermTarget) -> ArcTermTargetList {
        let mut result_list = targets.to_vec();

        let pendings_spec = self.article_paths.iter().find(
            |x| x.0.value == target.get_article().get_value());

        if pendings_spec.is_none() {
            return result_list;
        }

        let pendings_path = pendings_spec.unwrap().1;

        let reduce_func = |agr: ArcTermTargetList, def: &ArticleDefine| -> ArcTermTargetList {
            self.merge_item_pendings(period, ruleset,
                                     contract_terms, position_terms,
                                     &agr, def)
        };

        result_list = pendings_path.iter().fold(result_list, reduce_func);

        result_list
    }

    fn merge_item_pendings(&self, period: &dyn IPeriod, ruleset: &dyn IBundleProps,
                           contract_terms: &ArcContractTermList, position_terms: &ArcPositionTermList,
                           targets: &ArcTermTargetList, article_defs: &ArticleDefine) -> ArcTermTargetList {
        let mut result_list: ArcTermTargetList = targets.to_vec();

        let init_targets: ArcTermTargetList = targets.into_iter().filter(|x| {
            x.get_article().get_value()==article_defs.get_code().get_value()
        }).map(|t| t.clone()).collect();

        let target_list = ResultBuilder::get_target_list(period, ruleset, &self.concept_model,
                                               contract_terms, position_terms,
                                               &init_targets, &article_defs.get_code(), &article_defs.get_role());

        result_list = ResultBuilder::merge_targets(result_list, target_list);

        result_list
    }
    fn merge_list_pendings(&self, period: &dyn IPeriod, ruleset: &dyn IBundleProps,
                           contract_terms: &ArcContractTermList, position_terms: &ArcPositionTermList,
                           targets: ArcTermTargetList, calc_defs: &Vec<ArticleDefine>) -> ArcTermTargetList {
        let mut result_list: ArcTermTargetList = targets.to_vec();

        let define_list: Vec<ArticleDefine> = calc_defs.iter().filter(|x| targets.iter()
            .find(|t| t.get_article() == x.get_code()).is_none()).map(|d| d.clone()).collect();

        let target_list = self.add_defines_to_targets(period, ruleset,
                                                      contract_terms, position_terms,
                                                      targets, &define_list);

        result_list = ResultBuilder::merge_targets(result_list, target_list);

        result_list
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
        Ordering::Equal
    }
    fn target_finder_fun(target: &ArcTermTarget) -> impl FnMut(&ArticleCode) -> bool + '_ {
        move |x| {
            x.value == target.get_article().get_value()
        }
    }
}

impl IResultBuilder for ResultBuilder {
    fn is_valid(&self) -> bool {
        self.version.get_value() != 0
    }

    fn get_version(&self) -> &VersionCode {
        &self.version
    }

    fn get_period_init(&self) -> &dyn IPeriod {
        &self.period_init
    }

    fn init_with_period(&mut self,
                        version: &VersionCode,
                        period: &dyn IPeriod,
                        article_factory: &Box<dyn IArticleSpecFactory>,
                        concept_factory: &Box<dyn IConceptSpecFactory>) -> bool {
        self.version = VersionCode::get(version.get_value());
        self.period_init = Period::get(period.get_code());

        self.article_model = article_factory.get_spec_list(period, version);

        self.concept_model = concept_factory.get_spec_list(period, version);

        let (order, paths) = DependencyGraph::init_graph_model(&self.article_model, &self.concept_model);

        self.article_order = order;
        self.article_paths = paths;

        true
    }

    fn order(&self) -> &Vec<ArticleCode> {
        &self.article_order
    }

    fn paths(&self) -> &HashMap<ArticleCode, Vec<ArticleDefine>> {
        &self.article_paths
    }

    fn get_results(&self, ruleset: &dyn IBundleProps,
                   contracts: &ArcContractTermList, positions: &ArcPositionTermList,
                   targets: &ArcTermTargetList, calc_arts: &Vec<ArticleCode>) -> ResultArcTermResultList {
        let calcul_targets = self.build_calculs_list(&self.period_init, ruleset,
                                                     contracts, positions,
                                                     targets, calc_arts);

        let calcul_results = Self::build_results_list(&self.period_init, ruleset, calcul_targets);

        calcul_results
    }
}

