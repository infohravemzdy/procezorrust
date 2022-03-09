use legalios::service::period::IPeriod;
use legalios::service::bundle_props::IBundleProps;
use crate::registry_providers::article_provider::{BoxArticleSpec};
use crate::registry_providers::concept_provider::ResultFunc;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::term_symbol::ITermSymbol;
use crate::service_types::term_target::{ArcTermTarget};
use crate::service_types::term_result::ResultArcTermResultList;
use crate::service_errors::term_result_error::TermResultError;

pub(crate) trait ITermCalcul : ITermSymbol {
    fn ger_spec(&self) -> Option<BoxArticleSpec>;
    fn get_results(&self, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList;
}

pub(crate) type BoxTermCalcul = Box<dyn ITermCalcul>;

pub(crate) type BoxTermCalculList = Vec<BoxTermCalcul>;

pub(crate) struct TermCalcul {
    spec: Option<BoxArticleSpec>,
    target: ArcTermTarget,
    result_delegate: Option<ResultFunc>,
}

impl TermCalcul {
    pub(crate) fn new(_target: &ArcTermTarget, _spec: Option<BoxArticleSpec>, _delegate: Option<ResultFunc>) -> TermCalcul {
        TermCalcul {
            target: _target.clone(),
            spec: _spec,
            result_delegate: _delegate,
        }
    }
}

impl ITermSymbol for TermCalcul {
    fn is_null(&self) -> bool {
        self.get_article().get_value()==0
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

impl ITermCalcul for TermCalcul {
    fn ger_spec(&self) -> Option<BoxArticleSpec> {
        self.spec
    }

    fn get_results(&self, period: &dyn IPeriod, ruleset: &dyn IBundleProps, results: &ResultArcTermResultList) -> ResultArcTermResultList {
        if self.result_delegate.is_none() {
            let result_error = TermResultError::no_result_func_error(period, &self.target);
            return vec![Err(result_error)];
        }
        let result_target = self.result_delegate.unwrap()(self.target.clone(), self.spec, period, ruleset, results);

        result_target
    }
}
