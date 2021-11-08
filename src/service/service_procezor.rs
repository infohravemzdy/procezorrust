use legalios::service::period::IPeriod;
use legalios::factories::bundle_props::IBundleProps;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::term_result::{ResultArcTermResultList};
use crate::service_types::term_target::{ArcTermTargetList};
use crate::service_types::version_code::VersionCode;
use crate::service_types::article_define::{ArticleDefine, IArticleDefine};
use crate::registry_factories::article_factory::IArticleSpecFactory;
use crate::registry_factories::concept_factory::IConceptSpecFactory;
use crate::registry_providers::article_provider::BoxArticleSpec;
use crate::registry_providers::concept_provider::BoxConceptSpec;
use crate::registry::result_builder::{IResultBuilder, ResultBuilder};

pub(crate) trait IServiceProcezor {
    fn get_article_spec(&self, _article: &ArticleCode, _period: &dyn IPeriod, _version: &VersionCode) -> BoxArticleSpec;
    fn get_concept_spec(&self, _concept: &ConceptCode, _period: &dyn IPeriod, _version: &VersionCode) -> BoxConceptSpec;
    fn init_with_period(&mut self, _period: &dyn IPeriod) -> bool;
    fn get_results(&mut self, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, targets: &ArcTermTargetList) -> ResultArcTermResultList;
}

pub(crate) struct ServiceProcezor {
    version: VersionCode,
    fin_defs: ArticleDefine,
    article_factory: Box<dyn IArticleSpecFactory>,
    concept_factory: Box<dyn IConceptSpecFactory>,
    builder: Box<dyn IResultBuilder>,
}

impl IServiceProcezor for ServiceProcezor {
    fn get_article_spec(&self, _article: &ArticleCode, _period: &dyn IPeriod, _version: &VersionCode) -> BoxArticleSpec {
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
        init_result
    }

    fn get_results(&mut self, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, targets: &ArcTermTargetList) -> ResultArcTermResultList {
        let mut results: ResultArcTermResultList = vec![];

        let success: bool = self.init_with_period(_period);

        if !success {
            return results;
        }
        results = self.builder.get_results(_ruleset, targets, self.fin_defs);

        return results;
    }
}

#[allow(dead_code)]
type ArticleFactoryFunc = fn() -> Box<dyn IArticleSpecFactory>;

#[allow(dead_code)]
type ConceptFactoryFunc = fn() -> Box<dyn IConceptSpecFactory>;


#[allow(dead_code)]
impl ServiceProcezor {
    pub(crate) fn new(_version: i32, _fin_defs: &dyn IArticleDefine,
                      article_build_func: ArticleFactoryFunc,
                      concept_build_func: ConceptFactoryFunc) -> ServiceProcezor {
        ServiceProcezor{
            version: VersionCode::get(_version),
            fin_defs: ArticleDefine::from_defs(_fin_defs),
            article_factory: article_build_func(),
            concept_factory: concept_build_func(),
            builder: Box::new(ResultBuilder::new()),
        }
    }
}