use std::collections::HashMap;
use legalios::service::period::IPeriod;
use legalios::service::bundle_props::IBundleProps;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::term_result::{ResultArcTermResultList};
use crate::service_types::term_target::{ArcTermTargetList};
use crate::service_types::version_code::VersionCode;
use crate::service_types::article_define::{ArticleDefine};
use crate::registry_factories::article_factory::IArticleSpecFactory;
use crate::registry_factories::concept_factory::IConceptSpecFactory;
use crate::registry_providers::article_provider::ArcArticleSpec;
use crate::registry_providers::concept_provider::BoxConceptSpec;
use crate::registry::result_builder::{IResultBuilder, ResultBuilder};
use crate::service_types::article_term::ArticleTerm;
use crate::service_types::contract_term::ArcContractTermList;
use crate::service_types::position_term::ArcPositionTermList;

pub trait IServiceProcezor {
    fn builder_order(&self) -> &Vec<ArticleTerm>;
    fn builder_paths(&self) -> &HashMap<ArticleTerm, Vec<ArticleDefine>>;

    fn get_contract_terms(&self, _period: &dyn IPeriod, targets: &ArcTermTargetList) -> ArcContractTermList;
    fn get_position_terms(&self, _period: &dyn IPeriod, contracts: &ArcContractTermList, targets: &ArcTermTargetList) -> ArcPositionTermList;
    fn get_results(&mut self, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, targets: &ArcTermTargetList) -> ResultArcTermResultList;
    fn get_article_spec(&self, _article: &ArticleCode, _period: &dyn IPeriod, _version: &VersionCode) -> ArcArticleSpec;
    fn get_concept_spec(&self, _concept: &ConceptCode, _period: &dyn IPeriod, _version: &VersionCode) -> BoxConceptSpec;
    fn init_with_period(&mut self, _period: &dyn IPeriod) -> bool;
}

pub struct ServiceProcezor {
    version: VersionCode,
    calc_articles: Vec<ArticleCode>,
    article_factory: Box<dyn IArticleSpecFactory>,
    concept_factory: Box<dyn IConceptSpecFactory>,
    builder: Box<dyn IResultBuilder>,
}

impl IServiceProcezor for ServiceProcezor {
    fn builder_order(&self) -> &Vec<ArticleTerm> {
        self.builder.order()
    }

    fn builder_paths(&self) -> &HashMap<ArticleTerm, Vec<ArticleDefine>> {
        self.builder.paths()
    }

    fn get_contract_terms(&self, _period: &dyn IPeriod, _targets: &ArcTermTargetList) -> ArcContractTermList {
        vec![]
    }

    fn get_position_terms(&self, _period: &dyn IPeriod, _contracts: &ArcContractTermList, _targets: &ArcTermTargetList) -> ArcPositionTermList {
        vec![]
    }

    fn get_results(&mut self, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, targets: &ArcTermTargetList) -> ResultArcTermResultList {
        let mut results: ResultArcTermResultList = vec![];

        let success: bool = self.init_with_period(_period);

        if !success {
            return results;
        }
        let contract_terms = self.get_contract_terms(_period, targets);
        let position_terms = self.get_position_terms(_period, &contract_terms, targets);

        results = self.builder.get_results(_ruleset,
                                           &contract_terms,
                                           &position_terms, targets, &self.calc_articles);

        return results;
    }

    fn get_article_spec(&self, _article: &ArticleCode, _period: &dyn IPeriod, _version: &VersionCode) -> ArcArticleSpec {
        self.article_factory.get_spec(_article, _period, _version)
    }

    fn get_concept_spec(&self, _concept: &ConceptCode, _period: &dyn IPeriod, _version: &VersionCode) -> BoxConceptSpec {
        self.concept_factory.get_spec(_concept, _period, _version)
    }

    fn init_with_period(&mut self, period: &dyn IPeriod) -> bool {
        let curr_period_code = self.builder.get_period_init().get_code();
        let init_period_code = period.get_code();
        let init_builder: bool = curr_period_code != init_period_code || curr_period_code == 0;

        let mut init_result: bool = false;
        if self.builder.is_valid() {
            init_result = true;
        }
        if init_builder {
            init_result = self.builder.init_with_period(&self.version, period,
                                                        &self.article_factory,
                                                        &self.concept_factory);
        }
        if init_result == false {
            println!("Period: {}, init with period failed", period.get_code())
        }
        init_result
    }
}

#[allow(dead_code)]
type ArticleFactoryFunc = fn() -> Box<dyn IArticleSpecFactory>;

#[allow(dead_code)]
type ConceptFactoryFunc = fn() -> Box<dyn IConceptSpecFactory>;


#[allow(dead_code)]
impl ServiceProcezor {
    pub fn new(_version: i32, _calc_arts: &Vec<ArticleCode>,
                      article_build_func: ArticleFactoryFunc,
                      concept_build_func: ConceptFactoryFunc) -> ServiceProcezor {
        ServiceProcezor{
            version: VersionCode::get(_version),
            calc_articles: _calc_arts.into_iter().map(|x| x.clone()).collect(),
            article_factory: article_build_func(),
            concept_factory: concept_build_func(),
            builder: Box::new(ResultBuilder::new()),
        }
    }
}