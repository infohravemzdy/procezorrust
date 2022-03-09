use std::sync::Arc;
use chrono::{Datelike, NaiveDate, Weekday};
use crate::registry_constants::term_constants::{ITermConstants, TermConstants};
use crate::service_types::contract_code::ContractCode;

pub(crate) trait IContractTerm {
    fn get_contract(&self) -> ContractCode;
    fn get_date_from(&self) -> NaiveDate;
    fn get_date_stop(&self) -> NaiveDate;
    fn get_term_day_from(&self) -> i8;
    fn get_term_day_stop(&self) -> i8;
    fn is_valid(&self) -> bool;
    fn is_active(&self) -> bool;
}

pub(crate) type ArcContractTerm = Arc<dyn IContractTerm>;

pub(crate) type ArcContractTermList = Vec<ArcContractTerm>;

#[derive(Debug, Copy, Clone)]
pub(crate) struct ContractTerm {
    contract: ContractCode,
    date_from: NaiveDate,
    date_stop: NaiveDate,
    term_day_from: i8,
    term_day_stop: i8,
}

impl IContractTerm for ContractTerm {
    fn get_contract(&self) -> ContractCode {
        self.contract
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
        self.contract.get_value()!=0
    }
    fn is_active(&self) -> bool {
        (self.term_day_from < TermConstants::TERM_BEG_FINISHED
            && self.term_day_stop > TermConstants::TERM_END_FINISHED)
    }
}