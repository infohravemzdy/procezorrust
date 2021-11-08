use std::sync::Arc;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::article_define::ArticleDefine;
use crate::service_types::term_symbol::ITermSymbol;

pub(crate) trait ITermTarget : ITermSymbol {
    fn get_concept(&self) -> ConceptCode;
    fn get_target_basis(&self) -> i32;
    fn get_target_descr(&self) -> String;
    fn get_defs(&self) -> ArticleDefine;
    fn get_concept_descr(&self) -> String;
}

pub(crate) type ArcTermTarget = Arc<dyn ITermTarget>;

pub(crate) type ArcTermTargetList = Vec<ArcTermTarget>;

#[derive(Debug, Clone)]
pub(crate) struct TermTarget {
    month_code: MonthCode,
    contract: ContractCode,
    position: PositionCode,
    variant: VariantCode,
    article: ArticleCode,
    concept: ConceptCode,
    target_basis: i32,
    target_descr: String,
}

impl ITermSymbol for TermTarget {
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

impl ITermTarget for TermTarget {
    fn get_concept(&self) -> ConceptCode {
        self.concept
    }

    fn get_target_basis(&self) -> i32 {
        self.target_basis
    }

    fn get_target_descr(&self) -> String {
        self.target_descr.clone()
    }

    fn get_defs(&self) -> ArticleDefine {
        ArticleDefine::get(self.get_article().get_value(), self.get_concept().get_value())
    }

    fn get_concept_descr(&self) -> String {
        format!("ConceptCode for {}", self.concept.value)
    }
}

#[allow(dead_code)]
impl TermTarget {
    pub(crate) fn new(month: &MonthCode, contract: &ContractCode, position: &PositionCode, variant: &VariantCode,
                      article: &ArticleCode, concept: &ConceptCode, _basis: i32, _descr: &str) -> TermTarget {
        TermTarget {
            month_code: month.clone(),
            contract: contract.clone(),
            position: position.clone(),
            variant: variant.clone(),
            article: article.clone(),
            concept: concept.clone(),
            target_basis: _basis,
            target_descr: String::from(_descr),
        }
    }
    pub(crate) fn zero_value(month: &MonthCode, contract: &ContractCode, position: &PositionCode, variant: &VariantCode,
                      article: &ArticleCode, concept: &ConceptCode) -> TermTarget {
        TermTarget::new(month, contract, position, variant, article, concept, 0, "")
    }
    pub(crate) fn from_target(target: &ArcTermTarget) -> TermTarget {
        TermTarget::new(&target.get_month_code(),
                        &target.get_contract(),
                        &target.get_position(),
                        &target.get_variant(),
                        &target.get_article(),
                        &target.get_concept(),
                        target.get_target_basis(),
                        &target.get_target_descr())
    }
}

#[macro_export]
macro_rules! impl_target_term {
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

        impl ITermTarget for $t {
            fn get_concept(&self) -> ConceptCode {
                self.$p.concept
            }

            fn get_target_basis(&self) -> i32 {
                self.$p.target_basis
            }

            fn get_target_descr(&self) -> String {
                self.$p.target_descr.clone()
            }

            fn get_defs(&self) -> ArticleDefine {
                ArticleDefine::get(self.$p.get_article().get_value(), self.$p.get_concept().get_value())
            }

            fn get_concept_descr(&self) -> String {
                self.$p.get_concept_descr()
            }
        }
    }
}

