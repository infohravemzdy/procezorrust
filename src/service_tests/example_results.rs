use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::term_symbol::ITermSymbol;
use crate::service_types::term_target::ArcTermTarget;
use crate::registry_providers::article_provider::ArcArticleSpec;
use crate::service_types::term_result::{ITermResult, TermResult};

pub(crate) struct TimeshtWorkingResult {
    term: TermResult,
}

#[allow(dead_code)]
impl TimeshtWorkingResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> TimeshtWorkingResult {
        TimeshtWorkingResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(TimeshtWorkingResult, term);
pub(crate) struct AmountBasisResult {
    term: TermResult,
}

#[allow(dead_code)]
impl AmountBasisResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> AmountBasisResult {
        AmountBasisResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(AmountBasisResult, term);
pub(crate) struct AmountFixedResult {
    term: TermResult,
}

#[allow(dead_code)]
impl AmountFixedResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> AmountFixedResult {
        AmountFixedResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(AmountFixedResult, term);
pub(crate) struct HealthInsbaseResult {
    term: TermResult,
}

#[allow(dead_code)]
impl HealthInsbaseResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> HealthInsbaseResult {
        HealthInsbaseResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(HealthInsbaseResult, term);
pub(crate) struct SocialInsbaseResult {
    term: TermResult,
}

#[allow(dead_code)]
impl SocialInsbaseResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> SocialInsbaseResult {
        SocialInsbaseResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(SocialInsbaseResult, term);
pub(crate) struct HealthInspaymResult {
    term: TermResult,
}

#[allow(dead_code)]
impl HealthInspaymResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> HealthInspaymResult {
        HealthInspaymResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(HealthInspaymResult, term);
pub(crate) struct SocialInspaymResult {
    term: TermResult,
}

#[allow(dead_code)]
impl SocialInspaymResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> SocialInspaymResult {
        SocialInspaymResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(SocialInspaymResult, term);
pub(crate) struct TaxingAdvbaseResult {
    term: TermResult,
}

#[allow(dead_code)]
impl TaxingAdvbaseResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> TaxingAdvbaseResult {
        TaxingAdvbaseResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(TaxingAdvbaseResult, term);
pub(crate) struct TaxingAdvpaymResult {
    term: TermResult,
}

#[allow(dead_code)]
impl TaxingAdvpaymResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> TaxingAdvpaymResult {
        TaxingAdvpaymResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(TaxingAdvpaymResult, term);
pub(crate) struct IncomeGrossResult {
    term: TermResult,
}

#[allow(dead_code)]
impl IncomeGrossResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> IncomeGrossResult {
        IncomeGrossResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(IncomeGrossResult, term);
pub(crate) struct IncomeNettoResult {
    term: TermResult,
}

#[allow(dead_code)]
impl IncomeNettoResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> IncomeNettoResult {
        IncomeNettoResult {
            term: TermResult::new(target, spec),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(IncomeNettoResult, term);
