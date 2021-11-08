use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::term_symbol::ITermSymbol;
use crate::service_types::term_target::ArcTermTarget;
use crate::service_types::term_result::{IResultConst, ITermResult, TermResult};

pub(crate) struct TimeshtWorkingResult {
    term: TermResult,
}

#[allow(dead_code)]
impl TimeshtWorkingResult {
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> TimeshtWorkingResult {
        TimeshtWorkingResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> TimeshtWorkingResult {
        TimeshtWorkingResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> AmountBasisResult {
        AmountBasisResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> AmountBasisResult {
        AmountBasisResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> AmountFixedResult {
        AmountFixedResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> AmountFixedResult {
        AmountFixedResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> HealthInsbaseResult {
        HealthInsbaseResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> HealthInsbaseResult {
        HealthInsbaseResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> SocialInsbaseResult {
        SocialInsbaseResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> SocialInsbaseResult {
        SocialInsbaseResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> HealthInspaymResult {
        HealthInspaymResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> HealthInspaymResult {
        HealthInspaymResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> SocialInspaymResult {
        SocialInspaymResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> SocialInspaymResult {
        SocialInspaymResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> TaxingAdvbaseResult {
        TaxingAdvbaseResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> TaxingAdvbaseResult {
        TaxingAdvbaseResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> TaxingAdvpaymResult {
        TaxingAdvpaymResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> TaxingAdvpaymResult {
        TaxingAdvpaymResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> IncomeGrossResult {
        IncomeGrossResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> IncomeGrossResult {
        IncomeGrossResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
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
    pub(crate) fn new(target: ArcTermTarget, value: i32, basis: i32, descr: &str) -> IncomeNettoResult {
        IncomeNettoResult {
            term: TermResult::new(target, value, basis, descr),
        }
    }
    pub(crate) fn empty(_target: ArcTermTarget) -> IncomeNettoResult {
        IncomeNettoResult {
            term: TermResult::new(_target,
                                  TermResult::VALUE_ZERO,
                                  TermResult::BASIS_ZERO,
                                  TermResult::DESCRIPTION_EMPTY),
        }
    }
}

#[macro_use(crate::impl_result_term)]
crate::impl_result_term!(IncomeNettoResult, term);
