use std::fmt;
use legalios::service::period::{IPeriod, Period};
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::term_symbol::ITermSymbol;
use crate::service_types::term_target::{ArcTermTarget, ITermTarget};

pub struct TermResultError {
    message: String,
    target: ArcTermTarget,
    period: Period,
}

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[allow(dead_code)]
impl TermResultError {
    pub(crate) fn get_message(&self) -> String {
        String::from(&self.message)
    }
    pub(crate) fn extract_result_error(period: &dyn IPeriod, target: &ArcTermTarget) -> TermResultError {
        TermResultError {
            message: String::from("extraction error"),
            target: target.clone(),
            period: Period::get(period.get_code()),
        }
    }
    pub(crate) fn no_implementation_error(period: &dyn IPeriod, target: &ArcTermTarget) -> TermResultError {
        TermResultError {
            message: String::from("no implementation"),
            target: target.clone(),
            period: Period::get(period.get_code()),
        }
    }
    pub(crate) fn no_result_func_error(period: &dyn IPeriod, target: &ArcTermTarget) -> TermResultError {
        TermResultError {
            message: String::from("no result calculation function"),
            target: target.clone(),
            period: Period::get(period.get_code()),
        }
    }
    pub(crate) fn invalid_result_error(period: &dyn IPeriod, target: &ArcTermTarget, type_descr: String) -> TermResultError {
        TermResultError {
            message: format!("invalid result type {} error!", type_descr),
            target: target.clone(),
            period: Period::get(period.get_code()),
        }
    }
    pub(crate) fn invalid_ruleset_error(period: &dyn IPeriod, target: &ArcTermTarget, type_descr: String) -> TermResultError {
        TermResultError {
            message: format!("invalid {} Ruleset error!", type_descr),
            target: target.clone(),
            period: Period::get(period.get_code()),
        }
    }
    pub(crate) fn invalid_target_error(period: &dyn IPeriod, target: &ArcTermTarget, type_descr: String) -> TermResultError {
        TermResultError {
            message: format!("invalid target type {} error!", type_descr),
            target: target.clone(),
            period: Period::get(period.get_code()),
        }
    }
    pub(crate) fn no_result_found_error(period: &dyn IPeriod, target: &ArcTermTarget, article_descr: String,
                                        contract: &ContractCode, position: &PositionCode) -> TermResultError {
        TermResultError {
            message: format!("result for {}{} Not Found", article_descr, TermResultError::message_contract_position(contract, position)),
            target: target.clone(),
            period: Period::get(period.get_code()),
        }
    }

    pub(crate) fn null_result_found_error(period: &dyn IPeriod, target: &ArcTermTarget, article_descr: String,
                                          contract: &ContractCode, position: &PositionCode) -> TermResultError {
        TermResultError {
            message: format!("result found for {}{} but Instance is Null!", article_descr, TermResultError::message_contract_position(contract, position)),
            target: target.clone(),
            period: Period::get(period.get_code()),
        }
    }

    fn message_contract_position(contract: &ContractCode, position: &PositionCode) -> String {
        return if contract.is_valid() && position.is_valid() {
            format!(", contract={}, position={}", contract.get_value(), position.get_value())
        } else if contract.is_valid() {
            format!(", contract={}", contract.get_value())
        } else {
            String::from("")
        }
    }
}

impl Clone for TermResultError {
    fn clone(&self) -> Self {
        TermResultError {
            message: self.message.clone(),
            target: self.target.clone(),
            period: self.period.clone(),
        }
    }
}

impl ITermSymbol for TermResultError {
    fn is_null(&self) -> bool {
        self.target.is_null()
    }

    fn get_month_code(&self) -> MonthCode {
        self.target.get_month_code()
    }

    fn get_contract(&self) -> ContractCode {
        self.target.get_contract()
    }

    fn get_position(&self) -> PositionCode {
        self.target.get_position()
    }

    fn get_variant(&self) -> VariantCode {
        self.target.get_variant()
    }

    fn get_article(&self) -> ArticleCode {
        self.target.get_article()
    }

    fn get_article_descr(&self) -> String {
        self.target.get_article_descr()
    }
}

impl ITermTarget for TermResultError {
    fn get_concept(&self) -> ConceptCode {
        self.target.get_concept()
    }

    fn get_concept_descr(&self) -> String {
        self.target.get_concept_descr()
    }
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for TermResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
