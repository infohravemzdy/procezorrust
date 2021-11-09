use legalios::service::period::IPeriod;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::version_code::VersionCode;
use crate::service_types::article_define::{ArticleDefine, IArticleDefine};
use crate::registry_providers::article_provider::{ArticleSpec, ArticleSpecProvider, BoxArticleSpec, IArticleProvConst, IArticleSpec, IArticleSpecConst, IArticleSpecProvider};
use crate::service_tests::example_constants::{ExampleArticleConst, ExampleConceptConst};

pub(crate) struct TimeshtWorkingArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for TimeshtWorkingArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleTimeshtWorking as i32;
}

#[allow(dead_code)]
impl TimeshtWorkingArtProv {
    pub(crate) fn new() -> TimeshtWorkingArtProv {
        TimeshtWorkingArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(TimeshtWorkingArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(TimeshtWorkingArtProv, provider, TimeshtWorkingArtSpec);

pub(crate) struct TimeshtWorkingArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for TimeshtWorkingArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptTimeshtWorking as i32;
}

#[allow(dead_code)]
impl TimeshtWorkingArtSpec {
    fn from_code(_code: ArticleCode) -> TimeshtWorkingArtSpec {
        TimeshtWorkingArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(TimeshtWorkingArtSpec::CONCEPT_CODE), vec![]),
        }
    }
    fn new() -> TimeshtWorkingArtSpec {
        TimeshtWorkingArtSpec::from_code(ArticleCode::get(TimeshtWorkingArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(TimeshtWorkingArtSpec, spec);

pub(crate) struct PaymentSalaryArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for PaymentSalaryArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticlePaymentSalary as i32;
}

#[allow(dead_code)]
impl PaymentSalaryArtProv {
    pub(crate) fn new() -> PaymentSalaryArtProv {
        PaymentSalaryArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(PaymentSalaryArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(PaymentSalaryArtProv, provider, PaymentSalaryArtSpec);

pub(crate) struct PaymentSalaryArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for PaymentSalaryArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptAmountBasis as i32;
}

#[allow(dead_code)]
impl PaymentSalaryArtSpec {
    fn from_code(_code: ArticleCode) -> PaymentSalaryArtSpec {
        PaymentSalaryArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(PaymentSalaryArtSpec::CONCEPT_CODE),
                ArticleSpec::const_to_sums_array(vec![
                    ExampleArticleConst::ArticleIncomeGross as i32,
                    ExampleArticleConst::ArticleHealthInsbase as i32,
                    ExampleArticleConst::ArticleSocialInsbase as i32,
                    ExampleArticleConst::ArticleTaxingAdvbase as i32,
                ]),
            ),
        }
    }
    fn new() -> PaymentSalaryArtSpec {
        PaymentSalaryArtSpec::from_code(ArticleCode::get(PaymentSalaryArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(PaymentSalaryArtSpec, spec);

pub(crate) struct PaymentBonusArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for PaymentBonusArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticlePaymentBonus as i32;
}

#[allow(dead_code)]
impl PaymentBonusArtProv {
    pub(crate) fn new() -> PaymentBonusArtProv {
        PaymentBonusArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(PaymentBonusArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(PaymentBonusArtProv, provider, PaymentBonusArtSpec);

pub(crate) struct PaymentBonusArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for PaymentBonusArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptAmountFixed as i32;
}

#[allow(dead_code)]
impl PaymentBonusArtSpec {
    fn from_code(_code: ArticleCode) -> PaymentBonusArtSpec {
        PaymentBonusArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(PaymentBonusArtSpec::CONCEPT_CODE),
                ArticleSpec::const_to_sums_array(vec![
                    ExampleArticleConst::ArticleIncomeGross as i32,
                    ExampleArticleConst::ArticleHealthInsbase as i32,
                    ExampleArticleConst::ArticleSocialInsbase as i32,
                    ExampleArticleConst::ArticleTaxingAdvbase as i32,
                ]),
            ),
        }
    }
    fn new() -> PaymentBonusArtSpec {
        PaymentBonusArtSpec::from_code(ArticleCode::get(PaymentBonusArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(PaymentBonusArtSpec, spec);

pub(crate) struct PaymentBarterArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for PaymentBarterArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticlePaymentBarter as i32;
}

#[allow(dead_code)]
impl PaymentBarterArtProv {
    pub(crate) fn new() -> PaymentBarterArtProv {
        PaymentBarterArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(PaymentBarterArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(PaymentBarterArtProv, provider, PaymentBarterArtSpec);

pub(crate) struct PaymentBarterArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for PaymentBarterArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptAmountFixed as i32;
}

#[allow(dead_code)]
impl PaymentBarterArtSpec {
    fn from_code(_code: ArticleCode) -> PaymentBarterArtSpec {
        PaymentBarterArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(PaymentBarterArtSpec::CONCEPT_CODE),
                ArticleSpec::const_to_sums_array(vec![
                    ExampleArticleConst::ArticleHealthInsbase as i32,
                    ExampleArticleConst::ArticleSocialInsbase as i32,
                    ExampleArticleConst::ArticleTaxingAdvbase as i32,
                ]),
            ),
        }
    }
    fn new() -> PaymentBarterArtSpec {
        PaymentBarterArtSpec::from_code(ArticleCode::get(PaymentBarterArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(PaymentBarterArtSpec, spec);

pub(crate) struct AllowceHofficeArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for AllowceHofficeArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleAllowceHoffice as i32;
}

#[allow(dead_code)]
impl AllowceHofficeArtProv {
    pub(crate) fn new() -> AllowceHofficeArtProv {
        AllowceHofficeArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(AllowceHofficeArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(AllowceHofficeArtProv, provider, AllowceHofficeArtSpec);

pub(crate) struct AllowceHofficeArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for AllowceHofficeArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptAmountFixed as i32;
}

#[allow(dead_code)]
impl AllowceHofficeArtSpec {
    fn from_code(_code: ArticleCode) -> AllowceHofficeArtSpec {
        AllowceHofficeArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(AllowceHofficeArtSpec::CONCEPT_CODE),
                ArticleSpec::const_to_sums_array(vec![
                    ExampleArticleConst::ArticleIncomeNetto as i32,
                ]),
            ),
        }
    }
    fn new() -> AllowceHofficeArtSpec {
        AllowceHofficeArtSpec::from_code(ArticleCode::get(AllowceHofficeArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(AllowceHofficeArtSpec, spec);

pub(crate) struct HealthInsbaseArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for HealthInsbaseArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleHealthInsbase as i32;
}

#[allow(dead_code)]
impl HealthInsbaseArtProv {
    pub(crate) fn new() -> HealthInsbaseArtProv {
        HealthInsbaseArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(HealthInsbaseArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(HealthInsbaseArtProv, provider, HealthInsbaseArtSpec);

pub(crate) struct HealthInsbaseArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for HealthInsbaseArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptHealthInsbase as i32;
}

#[allow(dead_code)]
impl HealthInsbaseArtSpec {
    fn from_code(_code: ArticleCode) -> HealthInsbaseArtSpec {
        HealthInsbaseArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(HealthInsbaseArtSpec::CONCEPT_CODE), vec![]),
        }
    }
    fn new() -> HealthInsbaseArtSpec {
        HealthInsbaseArtSpec::from_code(ArticleCode::get(HealthInsbaseArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(HealthInsbaseArtSpec, spec);

pub(crate) struct SocialInsbaseArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for SocialInsbaseArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleSocialInsbase as i32;
}

#[allow(dead_code)]
impl SocialInsbaseArtProv {
    pub(crate) fn new() -> SocialInsbaseArtProv {
        SocialInsbaseArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(SocialInsbaseArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(SocialInsbaseArtProv, provider, SocialInsbaseArtSpec);

pub(crate) struct SocialInsbaseArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for SocialInsbaseArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptSocialInsbase as i32;
}

#[allow(dead_code)]
impl SocialInsbaseArtSpec {
    fn from_code(_code: ArticleCode) -> SocialInsbaseArtSpec {
        SocialInsbaseArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(SocialInsbaseArtSpec::CONCEPT_CODE), vec![]),
        }
    }
    fn new() -> SocialInsbaseArtSpec {
        SocialInsbaseArtSpec::from_code(ArticleCode::get(SocialInsbaseArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(SocialInsbaseArtSpec, spec);

pub(crate) struct HealthInspaymArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for HealthInspaymArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleHealthInspaym as i32;
}

#[allow(dead_code)]
impl HealthInspaymArtProv {
    pub(crate) fn new() -> HealthInspaymArtProv {
        HealthInspaymArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(HealthInspaymArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(HealthInspaymArtProv, provider, HealthInspaymArtSpec);

pub(crate) struct HealthInspaymArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for HealthInspaymArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptHealthInspaym as i32;
}

#[allow(dead_code)]
impl HealthInspaymArtSpec {
    fn from_code(_code: ArticleCode) -> HealthInspaymArtSpec {
        HealthInspaymArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(HealthInspaymArtSpec::CONCEPT_CODE), vec![]),
        }
    }
    fn new() -> HealthInspaymArtSpec {
        HealthInspaymArtSpec::from_code(ArticleCode::get(HealthInspaymArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(HealthInspaymArtSpec, spec);

pub(crate) struct SocialInspaymArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for SocialInspaymArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleSocialInspaym as i32;
}

#[allow(dead_code)]
impl SocialInspaymArtProv {
    pub(crate) fn new() -> SocialInspaymArtProv {
        SocialInspaymArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(SocialInspaymArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(SocialInspaymArtProv, provider, SocialInspaymArtSpec);

pub(crate) struct SocialInspaymArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for SocialInspaymArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptSocialInspaym as i32;
}

#[allow(dead_code)]
impl SocialInspaymArtSpec {
    fn from_code(_code: ArticleCode) -> SocialInspaymArtSpec {
        SocialInspaymArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(SocialInspaymArtSpec::CONCEPT_CODE), vec![]),
        }
    }
    fn new() -> SocialInspaymArtSpec {
        SocialInspaymArtSpec::from_code(ArticleCode::get(SocialInspaymArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(SocialInspaymArtSpec, spec);

pub(crate) struct TaxingAdvbaseArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for TaxingAdvbaseArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleTaxingAdvbase as i32;
}

#[allow(dead_code)]
impl TaxingAdvbaseArtProv {
    pub(crate) fn new() -> TaxingAdvbaseArtProv {
        TaxingAdvbaseArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(TaxingAdvbaseArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(TaxingAdvbaseArtProv, provider, TaxingAdvbaseArtSpec);

pub(crate) struct TaxingAdvbaseArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for TaxingAdvbaseArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptTaxingAdvbase as i32;
}

#[allow(dead_code)]
impl TaxingAdvbaseArtSpec {
    fn from_code(_code: ArticleCode) -> TaxingAdvbaseArtSpec {
        TaxingAdvbaseArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(TaxingAdvbaseArtSpec::CONCEPT_CODE), vec![]),
        }
    }
    fn new() -> TaxingAdvbaseArtSpec {
        TaxingAdvbaseArtSpec::from_code(ArticleCode::get(TaxingAdvbaseArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(TaxingAdvbaseArtSpec, spec);

pub(crate) struct TaxingAdvpaymArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for TaxingAdvpaymArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleTaxingAdvpaym as i32;
}

#[allow(dead_code)]
impl TaxingAdvpaymArtProv {
    pub(crate) fn new() -> TaxingAdvpaymArtProv {
        TaxingAdvpaymArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(TaxingAdvpaymArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(TaxingAdvpaymArtProv, provider, TaxingAdvpaymArtSpec);

pub(crate) struct TaxingAdvpaymArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for TaxingAdvpaymArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptTaxingAdvpaym as i32;
}

#[allow(dead_code)]
impl TaxingAdvpaymArtSpec {
    fn from_code(_code: ArticleCode) -> TaxingAdvpaymArtSpec {
        TaxingAdvpaymArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(TaxingAdvpaymArtSpec::CONCEPT_CODE), vec![]),
        }
    }
    fn new() -> TaxingAdvpaymArtSpec {
        TaxingAdvpaymArtSpec::from_code(ArticleCode::get(TaxingAdvpaymArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(TaxingAdvpaymArtSpec, spec);

pub(crate) struct IncomeGrossArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for IncomeGrossArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleIncomeGross as i32;
}

#[allow(dead_code)]
impl IncomeGrossArtProv {
    pub(crate) fn new() -> IncomeGrossArtProv {
        IncomeGrossArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(IncomeGrossArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(IncomeGrossArtProv, provider, IncomeGrossArtSpec);

pub(crate) struct IncomeGrossArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for IncomeGrossArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptIncomeGross as i32;
}

#[allow(dead_code)]
impl IncomeGrossArtSpec {
    fn from_code(_code: ArticleCode) -> IncomeGrossArtSpec {
        IncomeGrossArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(IncomeGrossArtSpec::CONCEPT_CODE), vec![]),
        }
    }
    fn new() -> IncomeGrossArtSpec {
        IncomeGrossArtSpec::from_code(ArticleCode::get(IncomeGrossArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(IncomeGrossArtSpec, spec);

pub(crate) struct IncomeNettoArtProv {
    provider: ArticleSpecProvider,
}

impl IArticleProvConst for IncomeNettoArtProv {
    const ARTICLE_CODE: i32 = ExampleArticleConst::ArticleIncomeNetto as i32;
}

#[allow(dead_code)]
impl IncomeNettoArtProv {
    pub(crate) fn new() -> IncomeNettoArtProv {
        IncomeNettoArtProv {
            provider: ArticleSpecProvider::new(ArticleCode::get(IncomeNettoArtProv::ARTICLE_CODE)),
        }
    }
}

#[macro_use(crate::impl_article_prov)]
crate::impl_article_prov!(IncomeNettoArtProv, provider, IncomeNettoArtSpec);

pub(crate) struct IncomeNettoArtSpec {
    spec: ArticleSpec,
}

impl IArticleSpecConst for IncomeNettoArtSpec {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptIncomeNetto as i32;
}

#[allow(dead_code)]
impl IncomeNettoArtSpec {
    fn from_code(_code: ArticleCode) -> IncomeNettoArtSpec {
        IncomeNettoArtSpec {
            spec: ArticleSpec::new(
                _code,
                ConceptCode::get(IncomeNettoArtSpec::CONCEPT_CODE), vec![]),
        }
    }
    fn new() -> IncomeNettoArtSpec {
        IncomeNettoArtSpec::from_code(ArticleCode::get(IncomeNettoArtProv::ARTICLE_CODE))
    }
}

#[macro_use(crate::impl_article_spec)]
crate::impl_article_spec!(IncomeNettoArtSpec, spec);

