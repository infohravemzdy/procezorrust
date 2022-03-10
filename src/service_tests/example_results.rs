use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::term_symbol::ITermSymbol;
use crate::service_types::term_target::ArcTermTarget;
use crate::registry_providers::article_provider::ArcArticleSpec;
use crate::service_tests::example_concepts::ExampleTermResult;
use crate::service_types::term_result::{ITermResult};

// TimeshtWorking		TIMESHT_WORKING
pub(crate) struct TimeshtWorkingResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl TimeshtWorkingResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> TimeshtWorkingResult {
        TimeshtWorkingResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(TimeshtWorkingResult, term);

// AmountBasis		AMOUNT_BASIS
pub(crate) struct AmountBasisResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl AmountBasisResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> AmountBasisResult {
        AmountBasisResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(AmountBasisResult, term);

// AmountFixed		AMOUNT_FIXED
pub(crate) struct AmountFixedResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl AmountFixedResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> AmountFixedResult {
        AmountFixedResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(AmountFixedResult, term);

// HealthInsbase		HEALTH_INSBASE
pub(crate) struct HealthInsbaseResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl HealthInsbaseResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> HealthInsbaseResult {
        HealthInsbaseResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(HealthInsbaseResult, term);

// SocialInsbase		SOCIAL_INSBASE
pub(crate) struct SocialInsbaseResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl SocialInsbaseResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> SocialInsbaseResult {
        SocialInsbaseResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(SocialInsbaseResult, term);

// HealthInspaym		HEALTH_INSPAYM
pub(crate) struct HealthInspaymResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl HealthInspaymResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> HealthInspaymResult {
        HealthInspaymResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(HealthInspaymResult, term);

// SocialInspaym		SOCIAL_INSPAYM
pub(crate) struct SocialInspaymResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl SocialInspaymResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> SocialInspaymResult {
        SocialInspaymResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(SocialInspaymResult, term);

// TaxingAdvbase		TAXING_ADVBASE
pub(crate) struct TaxingAdvbaseResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl TaxingAdvbaseResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> TaxingAdvbaseResult {
        TaxingAdvbaseResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(TaxingAdvbaseResult, term);

// TaxingAdvpaym		TAXING_ADVPAYM
pub(crate) struct TaxingAdvpaymResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl TaxingAdvpaymResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> TaxingAdvpaymResult {
        TaxingAdvpaymResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(TaxingAdvpaymResult, term);

// IncomeGross		INCOME_GROSS
pub(crate) struct IncomeGrossResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl IncomeGrossResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> IncomeGrossResult {
        IncomeGrossResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(IncomeGrossResult, term);

// IncomeNetto		INCOME_NETTO
pub(crate) struct IncomeNettoResult {
    term: ExampleTermResult,
}

#[allow(dead_code)]
impl IncomeNettoResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> IncomeNettoResult {
        IncomeNettoResult {
            term: ExampleTermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(IncomeNettoResult, term);

