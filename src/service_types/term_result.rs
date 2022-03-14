use std::sync::Arc;
use crate::registry_providers::article_provider::{ArcArticleSpec};
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::term_symbol::ITermSymbol;
use crate::service_types::term_target::{ArcTermTarget};
use crate::service_errors::term_result_error::TermResultError;

pub trait ITermResult : ITermSymbol {
    fn get_target(&self) -> ArcTermTarget;
    fn get_spec(&self) -> ArcArticleSpec;
    fn get_concept(&self) -> ConceptCode;
    fn get_concept_descr(&self) -> String;
}

pub(crate) type ArcTermResult = Arc<dyn ITermResult>;

pub(crate) type ResultArcTermResult = Result<ArcTermResult, TermResultError>;

pub(crate) type ResultArcTermResultList = Vec<ResultArcTermResult>;

pub struct TermResult {
    target: ArcTermTarget,
    spec : ArcArticleSpec,
    month_code: MonthCode,
    contract: ContractCode,
    position: PositionCode,
    variant: VariantCode,
    article: ArticleCode,
    concept: ConceptCode,
}

impl ITermSymbol for TermResult {
    fn is_null(&self) -> bool {
        self.article.get_value()==0
    }

    fn get_month_code(&self) -> MonthCode {
        self.month_code
    }

    fn get_contract(&self) -> ContractCode {
        self.contract
    }

    fn get_position(&self) -> PositionCode {
        self.position
    }

    fn get_variant(&self) -> VariantCode {
        self.variant
    }

    fn get_article(&self) -> ArticleCode {
        self.article
    }

    fn get_article_descr(&self) -> String {
        format!("ArticleCode for {}", self.article.value)
    }
}

impl ITermResult for TermResult {
    fn get_target(&self) -> ArcTermTarget {
        self.target.clone()
    }

    fn get_spec(&self) -> ArcArticleSpec { self.spec.clone() }

    fn get_concept(&self) -> ConceptCode {
        self.concept
    }

    fn get_concept_descr(&self) -> String {
        format!("ConceptCode for {}", self.concept.value)
    }
}

impl TermResult {
    pub fn new(_target: ArcTermTarget, _spec: ArcArticleSpec) -> TermResult {
        let _month: MonthCode = _target.get_month_code().clone();
        let _contract: ContractCode = _target.get_contract().clone();
        let _position: PositionCode = _target.get_position().clone();
        let _variant: VariantCode = _target.get_variant().clone();
        let _article: ArticleCode = _target.get_article().clone();
        let _concept: ConceptCode = _target.get_concept().clone();

        TermResult {
            target: _target,
            spec: _spec,
            month_code: _month,
            contract: _contract,
            position: _position,
            variant: _variant,
            article: _article,
            concept: _concept,
        }
    }
}

#[macro_export]
macro_rules! impl_result_term {
    ($t:ident, $p:ident) => {
        impl ITermSymbol for $t {
            fn is_null(&self) -> bool {
                self.$p.is_null()
            }

            fn get_month_code(&self) -> MonthCode {
                self.$p.get_month_code()
            }

            fn get_contract(&self) -> ContractCode {
                self.$p.get_contract()
            }

            fn get_position(&self) -> PositionCode {
                self.$p.get_position()
            }

            fn get_variant(&self) -> VariantCode {
                self.$p.get_variant()
            }

            fn get_article(&self) -> ArticleCode {
                self.$p.get_article()
            }

            fn get_article_descr(&self) -> String {
                self.$p.get_article_descr()
            }
        }

        impl ITermResult for $t {
            fn get_target(&self) -> ArcTermTarget {
                self.$p.get_target()
            }

            fn get_spec(&self) -> ArcArticleSpec {
                self.$p.get_spec()
            }

            fn get_concept(&self) -> ConceptCode {
                self.$p.get_concept()
            }

            fn get_concept_descr(&self) -> String {
                self.$p.get_concept_descr()
            }
        }
    }
}

