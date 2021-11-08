use legalios::service::period::IPeriod;
use legalios::factories::bundle_props::IBundleProps;
use crate::service_types::version_code::VersionCode;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::article_define::IArticleDefine;
use crate::service_types::term_target::{ArcTermTargetList};
use crate::service_types::term_result::{ResultArcTermResultList};
use crate::registry_factories::article_factory::{ArticleSpecFactory, BoxArticleSpecFactory, IArticleSpecFactory, ProviderRecord};
use crate::registry_factories::concept_factory::{BoxConceptSpecFactory, ConceptSpecFactory, IConceptSpecFactory};
use crate::registry_providers::article_provider::{BoxArticleSpec, BoxArticleSpecProvider};
use crate::registry_providers::concept_provider::{BoxConceptSpec, BoxConceptSpecProvider};
use crate::service::service_procezor::{IServiceProcezor, ServiceProcezor};
use crate::service_tests::example_constants::ExampleArticleConst;
use crate::service_tests::example_constants::ExampleConceptConst;
use crate::service_tests::example_providers::{AmountBasisConProv, AmountFixedConProv, HealthInsbaseConProv, HealthInspaymConProv, IncomeGrossConProv, IncomeNettoConProv, SocialInsbaseConProv, SocialInspaymConProv, TaxingAdvbaseConProv, TaxingAdvpaymConProv, TimeshtWorkingConProv};

pub(crate) struct ExampleArticleFactory {
    factory: ArticleSpecFactory,
}

#[allow(dead_code)]
impl ExampleArticleFactory {
    fn new() -> ExampleArticleFactory {
        ExampleArticleFactory {
            factory: ArticleSpecFactory::new(Self::factory_builder),
        }
    }
    fn factory_builder() -> Vec<BoxArticleSpecProvider> {
        let providers_config = vec![
            ProviderRecord::new(ExampleArticleConst::ArticleTimeshtWorking as i32, ExampleConceptConst::ConceptTimeshtWorking as i32,
                                vec![]),
            ProviderRecord::new(ExampleArticleConst::ArticlePaymentSalary as i32, ExampleConceptConst::ConceptAmountBasis as i32,
                                vec![
                                    ExampleArticleConst::ArticleIncomeGross as i32,
                                    ExampleArticleConst::ArticleHealthInsbase as i32,
                                    ExampleArticleConst::ArticleSocialInsbase as i32,
                                    ExampleArticleConst::ArticleTaxingAdvbase as i32,
                                ]),
            ProviderRecord::new(ExampleArticleConst::ArticlePaymentBonus as i32, ExampleConceptConst::ConceptAmountFixed as i32,
                                vec![
                                    ExampleArticleConst::ArticleIncomeGross as i32,
                                    ExampleArticleConst::ArticleHealthInsbase as i32,
                                    ExampleArticleConst::ArticleSocialInsbase as i32,
                                    ExampleArticleConst::ArticleTaxingAdvbase as i32,
                                ]),
            ProviderRecord::new(ExampleArticleConst::ArticlePaymentBarter as i32, ExampleConceptConst::ConceptAmountFixed as i32,
                                vec![
                                    ExampleArticleConst::ArticleHealthInsbase as i32,
                                    ExampleArticleConst::ArticleSocialInsbase as i32,
                                    ExampleArticleConst::ArticleTaxingAdvbase as i32,
                                ]),
            ProviderRecord::new(ExampleArticleConst::ArticleAllowceHoffice as i32, ExampleConceptConst::ConceptAmountFixed as i32,
                                vec![
                                    ExampleArticleConst::ArticleIncomeNetto as i32,
                                ]),
            ProviderRecord::new(ExampleArticleConst::ArticleHealthInsbase as i32, ExampleConceptConst::ConceptHealthInsbase as i32,
                                vec![]),
            ProviderRecord::new(ExampleArticleConst::ArticleSocialInsbase as i32, ExampleConceptConst::ConceptSocialInsbase as i32,
                                vec![]),
            ProviderRecord::new(ExampleArticleConst::ArticleHealthInspaym as i32, ExampleConceptConst::ConceptHealthInspaym as i32,
                                vec![]),
            ProviderRecord::new(ExampleArticleConst::ArticleSocialInspaym as i32, ExampleConceptConst::ConceptSocialInspaym as i32,
                                vec![]),
            ProviderRecord::new(ExampleArticleConst::ArticleTaxingAdvbase as i32, ExampleConceptConst::ConceptTaxingAdvbase as i32,
                                vec![]),
            ProviderRecord::new(ExampleArticleConst::ArticleTaxingAdvpaym as i32, ExampleConceptConst::ConceptTaxingAdvpaym as i32,
                                vec![]),
            ProviderRecord::new(ExampleArticleConst::ArticleIncomeGross as i32, ExampleConceptConst::ConceptIncomeGross as i32,
                                vec![]),
            ProviderRecord::new(ExampleArticleConst::ArticleIncomeNetto as i32, ExampleConceptConst::ConceptIncomeNetto as i32,
                                vec![]),
        ];
        ArticleSpecFactory::build_providers_from_records(providers_config)
    }
}

impl IArticleSpecFactory for ExampleArticleFactory {
    fn get_spec(&self, code: &ArticleCode, period: &dyn IPeriod, version: &VersionCode) -> BoxArticleSpec {
        self.factory.get_spec(code, period, version)
    }

    fn get_spec_list(&self, period: &dyn IPeriod, version: &VersionCode) -> Vec<BoxArticleSpec> {
        self.factory.get_spec_list(period, version)
    }
}

pub(crate) struct ExampleConceptFactory {
    factory: ConceptSpecFactory,
}

#[allow(dead_code)]
impl ExampleConceptFactory {
    fn new() -> ExampleConceptFactory {
        ExampleConceptFactory {
            factory: ConceptSpecFactory::new(Self::factory_builder),
        }
    }
    fn factory_builder() -> Vec<BoxConceptSpecProvider> {
        vec![
            Box::new(TimeshtWorkingConProv::new()),
            Box::new(AmountBasisConProv::new()),
            Box::new(AmountFixedConProv::new()),
            Box::new(HealthInsbaseConProv::new()),
            Box::new(SocialInsbaseConProv::new()),
            Box::new(HealthInspaymConProv::new()),
            Box::new(SocialInspaymConProv::new()),
            Box::new(TaxingAdvbaseConProv::new()),
            Box::new(TaxingAdvpaymConProv::new()),
            Box::new(IncomeGrossConProv::new()),
            Box::new(IncomeNettoConProv::new()),
        ]
    }
}

impl IConceptSpecFactory for ExampleConceptFactory {
    fn get_spec(&self, code: &ConceptCode, period: &dyn IPeriod, version: &VersionCode) -> BoxConceptSpec {
        self.factory.get_spec(code, period, version)
    }

    fn get_spec_list(&self, period: &dyn IPeriod, version: &VersionCode) -> Vec<BoxConceptSpec> {
        self.factory.get_spec_list(period, version)
    }
}


pub(crate) struct ExampleService {
    service: ServiceProcezor,
}

impl IServiceProcezor for ExampleService {
    fn get_article_spec(&self, _article: &ArticleCode, _period: &dyn IPeriod, _version: &VersionCode) -> BoxArticleSpec {
        self.service.get_article_spec(_article, _period, _version)
    }

    fn get_concept_spec(&self, _concept: &ConceptCode, _period: &dyn IPeriod, _version: &VersionCode) -> BoxConceptSpec {
        self.service.get_concept_spec(_concept, _period, _version)
    }

    fn init_with_period(&mut self, _period: &dyn IPeriod) -> bool {
        self.service.init_with_period(_period)
    }

    fn get_results(&mut self, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, targets: &ArcTermTargetList) -> ResultArcTermResultList {
        self.service.get_results(_period, _ruleset, targets)
    }
}

#[allow(dead_code)]
impl ExampleService {
    pub(crate) fn new(_version: i32, fin_defs: &dyn IArticleDefine) -> ExampleService {
        let service = ExampleService {
            service: ServiceProcezor::new(
                _version, fin_defs,
                Self::article_factory_builder,
                Self::concept_factory_builder,
            ),
        };

        service
    }
    fn article_factory_builder() -> BoxArticleSpecFactory {
        Box::new(ExampleArticleFactory::new())
    }
    fn concept_factory_builder() -> BoxConceptSpecFactory {
        Box::new(ExampleConceptFactory::new())
    }
}