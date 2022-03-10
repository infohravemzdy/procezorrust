use std::sync::Arc;
use legalios::service::bundle_props::IBundleProps;
use legalios::service::period::IPeriod;
use crate::registry_providers::article_provider::ArcArticleSpec;
use crate::registry_providers::concept_provider::{ConceptSpec, IConceptSpec, ResultFunc};
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::term_symbol::ITermSymbol;
use crate::service_types::term_target::{ArcTermTarget, ArcTermTargetList, ITermTarget, TermTarget};
use crate::service_types::term_result::{ITermResult, TermResult};
use crate::service_tests::example_constants::ExampleArticleConst;
use crate::service_tests::example_constants::ExampleConceptConst;
use crate::service_types::concept_define::IConceptDefine;
use crate::service_types::contract_term::ArcContractTermList;
use crate::service_types::position_term::ArcPositionTermList;

pub(crate) struct ExampleConceptSpec {
    concept: ConceptSpec,
}

#[allow(dead_code)]
impl ExampleConceptSpec {
    pub(crate) fn new(_code: ConceptCode, _path: Vec<ArticleCode>, _result: Option<ResultFunc>) -> ExampleConceptSpec {
        ExampleConceptSpec {
            concept: ConceptSpec::new(_code, _path, _result),
        }
    }
}

impl IConceptDefine for ExampleConceptSpec {
    fn get_code(&self) -> ConceptCode {
        self.concept.get_code()
    }
}

#[allow(dead_code)]
impl IConceptSpec for ExampleConceptSpec {
    fn get_path(&self) -> Vec<ArticleCode> {
        self.concept.get_path()
    }

    fn get_result_delegate(&self) -> Option<ResultFunc> {
        self.concept.get_result_delegate()
    }

    fn default_target_list(&self, article: &ArticleCode, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, month: &MonthCode,
                           _contract_terms: &ArcContractTermList, _position_terms: &ArcPositionTermList,
                           targets: &ArcTermTargetList, vars: VariantCode) -> ArcTermTargetList {

        let con = ContractCode::zero();
        let pos = PositionCode::zero();

        if targets.len()!=0 {
            return vec![];
        }
        return vec![Arc::new(ExampleTermTarget::new(month, &con, &pos, &vars, &article, &self.get_code()))];
    }
}

pub(crate) struct ExampleTermTarget {
    term: TermTarget,
}

#[allow(dead_code)]
impl ExampleTermTarget {
    pub(crate) fn new(month: &MonthCode, contract: &ContractCode, position: &PositionCode, variant: &VariantCode,
                      article: &ArticleCode, concept: &ConceptCode) -> ExampleTermTarget {
        ExampleTermTarget {
            term: TermTarget::new(month, contract, position, variant, article, concept),
        }
    }
    pub(crate) fn zero_value(month: &MonthCode, contract: &ContractCode, position: &PositionCode, variant: &VariantCode,
                             article: &ArticleCode, concept: &ConceptCode) -> ExampleTermTarget {
        ExampleTermTarget::new(month, contract, position, variant, article, concept)
    }
}

impl ITermSymbol for ExampleTermTarget {
    fn is_null(&self) -> bool {
        self.term.is_null()
    }

    fn get_month_code(&self) -> MonthCode {
        self.term.get_month_code()
    }

    fn get_contract(&self) -> ContractCode {
        self.term.get_contract()
    }

    fn get_position(&self) -> PositionCode {
        self.term.get_position()
    }

    fn get_variant(&self) -> VariantCode {
        self.term.get_variant()
    }

    fn get_article(&self) -> ArticleCode {
        self.term.get_article()
    }

    fn get_article_descr(&self) -> String {
        ExampleArticleConst::name_of_article(self.get_article().value)
    }
}

impl ITermTarget for ExampleTermTarget {
    fn get_concept(&self) -> ConceptCode {
        self.term.get_concept()
    }

    fn get_concept_descr(&self) -> String {
        ExampleConceptConst::name_of_concept(self.get_concept().value)
    }
}

pub(crate) struct ExampleTermResult {
    term: TermResult,
}

#[allow(dead_code)]
impl ExampleTermResult {
    pub(crate) fn new(target: ArcTermTarget, spec: ArcArticleSpec) -> ExampleTermResult {
        ExampleTermResult {
            term: TermResult::new(target, spec.clone()),
        }
    }
}

impl ITermSymbol for ExampleTermResult {
    fn is_null(&self) -> bool {
        self.term.is_null()
    }

    fn get_month_code(&self) -> MonthCode {
        self.term.get_month_code()
    }

    fn get_contract(&self) -> ContractCode {
        self.term.get_contract()
    }

    fn get_position(&self) -> PositionCode {
        self.term.get_position()
    }

    fn get_variant(&self) -> VariantCode {
        self.term.get_variant()
    }

    fn get_article(&self) -> ArticleCode {
        self.term.get_article()
    }

    fn get_article_descr(&self) -> String {
        ExampleArticleConst::name_of_article(self.get_article().value)
    }
}

impl ITermResult for ExampleTermResult {
    fn get_target(&self) -> ArcTermTarget {
        self.term.get_target()
    }

    fn get_spec(&self) -> ArcArticleSpec {
        self.term.get_spec()
    }

    fn get_concept(&self) -> ConceptCode {
        self.term.get_concept()
    }

    fn get_concept_descr(&self) -> String {
        ExampleConceptConst::name_of_concept(self.get_concept().value)
    }
}

