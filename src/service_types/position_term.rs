use std::sync::Arc;
use chrono::NaiveDate;
use crate::registry_constants::term_constants::{ITermConstants, TermConstants};
use crate::service_types::contract_code::ContractCode;
use crate::service_types::contract_term::{ContractTerm, IContractTerm};
use crate::service_types::position_code::PositionCode;

pub trait IPositionTerm {
    fn get_contract(&self) -> ContractCode;
    fn get_position(&self) -> PositionCode;
    fn get_base_term(&self) -> Box<dyn IContractTerm>;
    fn get_date_from(&self) -> NaiveDate;
    fn get_date_stop(&self) -> NaiveDate;
    fn get_term_day_from(&self) -> i8;
    fn get_term_day_stop(&self) -> i8;
    fn is_valid(&self) -> bool;
    fn is_active(&self) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct PositionTerm {
    contract: ContractCode,
    position: PositionCode,
    base_term: ContractTerm,
    date_from: NaiveDate,
    date_stop: NaiveDate,
    term_day_from: i8,
    term_day_stop: i8,
}

pub(crate) type ArcPositionTerm = Arc<dyn IPositionTerm>;

pub(crate) type ArcPositionTermList = Vec<ArcPositionTerm>;


impl PositionTerm {
    fn is_position_active(&self) -> bool {
        self.term_day_from < TermConstants::TERM_BEG_FINISHED
            && self.term_day_stop > TermConstants::TERM_END_FINISHED
    }

}
impl IPositionTerm for PositionTerm {
    fn get_contract(&self) -> ContractCode {
        self.contract
    }

    fn get_position(&self) -> PositionCode {
        self.position
    }

    fn get_base_term(&self) -> Box<dyn IContractTerm> {
        Box::new(self.base_term.clone())
    }

    fn get_date_from(&self) -> NaiveDate {
        self.date_from
    }

    fn get_date_stop(&self) -> NaiveDate {
        self.date_stop
    }

    fn get_term_day_from(&self) -> i8 {
        self.term_day_from
    }

    fn get_term_day_stop(&self) -> i8 {
        self.term_day_stop
    }

    fn is_valid(&self) -> bool {
        self.position.get_value()!=0
    }

    fn is_active(&self) -> bool {
        if self.base_term.is_valid() {
            return self.base_term.is_active() && self.is_position_active()
        }
        return self.is_position_active()
    }
}