use std::sync::Arc;
use legalios::service::period::IPeriod;
use legalios::service::bundle_props::IBundleProps;
use crate::registry_providers::article_provider::BoxArticleSpec;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::concept_define::IConceptDefine;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;
use crate::service_types::version_code::VersionCode;
use crate::service_types::term_symbol::ITermSymbol;
use crate::service_types::term_target::{ArcTermTarget, ArcTermTargetList, ITermTarget, TermTarget};
use crate::service_types::term_result::{ITermResult, ResultArcTermResultList, TermResult};
use crate::registry_providers::concept_provider::{BoxConceptSpec, ConceptSpec, ConceptSpecProvider, IConceptSpec, IConceptSpecConst, IConceptSpecProvider, ResultFunc};
use crate::service_tests::example_constants::ExampleArticleConst;
use crate::service_tests::example_constants::ExampleConceptConst;
use crate::service_tests::example_results::{AmountBasisResult, AmountFixedResult, HealthInsbaseResult, HealthInspaymResult, IncomeGrossResult, IncomeNettoResult, SocialInsbaseResult, SocialInspaymResult, TaxingAdvbaseResult, TaxingAdvpaymResult, TimeshtWorkingResult};
use crate::service_types::article_define::ArticleDefine;
use crate::service_types::contract_term::ArcContractTermList;
use crate::service_types::position_term::ArcPositionTermList;

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
    pub(crate) fn new(target: ArcTermTarget, spec: Option<BoxArticleSpec>) -> ExampleTermResult {
        ExampleTermResult {
            term: TermResult::new(target, spec),
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

    fn get_spec(&self) -> Option<BoxArticleSpec> {
        self.term.get_spec()
    }

    fn get_concept(&self) -> ConceptCode {
        self.term.get_concept()
    }

    fn get_concept_descr(&self) -> String {
        ExampleConceptConst::name_of_concept(self.get_concept().value)
    }
}

pub(crate) struct TimeshtWorkingConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for TimeshtWorkingConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptTimeshtWorking as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(TimeshtWorkingConProv, provider, TimeshtWorkingConSpec);

pub(crate) struct TimeshtWorkingConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl TimeshtWorkingConSpec {
    fn from_code(_code: ConceptCode) -> TimeshtWorkingConSpec {
        TimeshtWorkingConSpec {
            spec: ConceptSpec::new(_code, vec![], Some(TimeshtWorkingConSpec::concept_eval)),
        }
    }

    fn new() -> TimeshtWorkingConSpec {
        TimeshtWorkingConSpec::from_code(ConceptCode::get(TimeshtWorkingConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = TimeshtWorkingResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl TimeshtWorkingConProv {
    pub(crate) fn new() -> TimeshtWorkingConProv {
        TimeshtWorkingConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(TimeshtWorkingConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(TimeshtWorkingConSpec, spec);

pub(crate) struct AmountBasisConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for AmountBasisConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptAmountBasis as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(AmountBasisConProv, provider, AmountBasisConSpec);

pub(crate) struct AmountBasisConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl AmountBasisConSpec {
    fn from_code(_code: ConceptCode) -> AmountBasisConSpec {
        let spec_path: Vec<i32> = vec![
            ExampleArticleConst::ArticleTimeshtWorking as i32,
        ];
        AmountBasisConSpec {
            spec: ConceptSpec::new(_code, ConceptSpec::const_to_path_array(spec_path), Some(AmountBasisConSpec::concept_eval)),
        }
    }

    fn new() -> AmountBasisConSpec {
        AmountBasisConSpec::from_code(ConceptCode::get(AmountBasisConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = AmountBasisResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl AmountBasisConProv {
    pub(crate) fn new() -> AmountBasisConProv {
        AmountBasisConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(AmountBasisConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(AmountBasisConSpec, spec);

pub(crate) struct AmountFixedConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for AmountFixedConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptAmountFixed as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(AmountFixedConProv, provider, AmountFixedConSpec);

pub(crate) struct AmountFixedConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl AmountFixedConSpec {
    fn from_code(_code: ConceptCode) -> AmountFixedConSpec {
        AmountFixedConSpec {
            spec: ConceptSpec::new(_code, vec![], Some(AmountFixedConSpec::concept_eval)),
        }
    }

    fn new() -> AmountFixedConSpec {
        AmountFixedConSpec::from_code(ConceptCode::get(AmountFixedConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = AmountFixedResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl AmountFixedConProv {
    pub(crate) fn new() -> AmountFixedConProv {
        AmountFixedConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(AmountFixedConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(AmountFixedConSpec, spec);

pub(crate) struct HealthInsbaseConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for HealthInsbaseConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptHealthInsbase as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(HealthInsbaseConProv, provider, HealthInsbaseConSpec);

pub(crate) struct HealthInsbaseConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl HealthInsbaseConSpec {
    fn from_code(_code: ConceptCode) -> HealthInsbaseConSpec {
        HealthInsbaseConSpec {
            spec: ConceptSpec::new(_code, vec![], Some(HealthInsbaseConSpec::concept_eval)),
        }
    }

    fn new() -> HealthInsbaseConSpec {
        HealthInsbaseConSpec::from_code(ConceptCode::get(HealthInsbaseConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = HealthInsbaseResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl HealthInsbaseConProv {
    pub(crate) fn new() -> HealthInsbaseConProv {
        HealthInsbaseConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(HealthInsbaseConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(HealthInsbaseConSpec, spec);

pub(crate) struct SocialInsbaseConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for SocialInsbaseConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptSocialInsbase as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(SocialInsbaseConProv, provider, SocialInsbaseConSpec);

pub(crate) struct SocialInsbaseConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl SocialInsbaseConSpec {
    fn from_code(_code: ConceptCode) -> SocialInsbaseConSpec {
        SocialInsbaseConSpec {
            spec: ConceptSpec::new(_code, vec![], Some(SocialInsbaseConSpec::concept_eval)),
        }
    }

    fn new() -> SocialInsbaseConSpec {
        SocialInsbaseConSpec::from_code(ConceptCode::get(SocialInsbaseConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = SocialInsbaseResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl SocialInsbaseConProv {
    pub(crate) fn new() -> SocialInsbaseConProv {
        SocialInsbaseConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(SocialInsbaseConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(SocialInsbaseConSpec, spec);

pub(crate) struct HealthInspaymConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for HealthInspaymConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptHealthInspaym as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(HealthInspaymConProv, provider, HealthInspaymConSpec);

pub(crate) struct HealthInspaymConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl HealthInspaymConSpec {
    fn from_code(_code: ConceptCode) -> HealthInspaymConSpec {
        let spec_path: Vec<i32> = vec![
            ExampleArticleConst::ArticleHealthInsbase as i32,
        ];
        HealthInspaymConSpec {
            spec: ConceptSpec::new(_code, ConceptSpec::const_to_path_array(spec_path), Some(HealthInspaymConSpec::concept_eval)),
        }
    }

    fn new() -> HealthInspaymConSpec {
        HealthInspaymConSpec::from_code(ConceptCode::get(HealthInspaymConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = HealthInspaymResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl HealthInspaymConProv {
    pub(crate) fn new() -> HealthInspaymConProv {
        HealthInspaymConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(HealthInspaymConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(HealthInspaymConSpec, spec);

pub(crate) struct SocialInspaymConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for SocialInspaymConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptSocialInspaym as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(SocialInspaymConProv, provider, SocialInspaymConSpec);

pub(crate) struct SocialInspaymConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl SocialInspaymConSpec {
    fn from_code(_code: ConceptCode) -> SocialInspaymConSpec {
        let spec_path: Vec<i32> = vec![
            ExampleArticleConst::ArticleSocialInsbase as i32,
        ];
        SocialInspaymConSpec {
            spec: ConceptSpec::new(_code, ConceptSpec::const_to_path_array(spec_path), Some(SocialInspaymConSpec::concept_eval)),
        }
    }

    fn new() -> SocialInspaymConSpec {
        SocialInspaymConSpec::from_code(ConceptCode::get(SocialInspaymConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = SocialInspaymResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl SocialInspaymConProv {
    pub(crate) fn new() -> SocialInspaymConProv {
        SocialInspaymConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(SocialInspaymConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(SocialInspaymConSpec, spec);

pub(crate) struct TaxingAdvbaseConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for TaxingAdvbaseConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptTaxingAdvbase as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(TaxingAdvbaseConProv, provider, TaxingAdvbaseConSpec);

pub(crate) struct TaxingAdvbaseConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl TaxingAdvbaseConSpec {
    fn from_code(_code: ConceptCode) -> TaxingAdvbaseConSpec {
        TaxingAdvbaseConSpec {
            spec: ConceptSpec::new(_code, vec![], Some(TaxingAdvbaseConSpec::concept_eval)),
        }
    }

    fn new() -> TaxingAdvbaseConSpec {
        TaxingAdvbaseConSpec::from_code(ConceptCode::get(TaxingAdvbaseConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = TaxingAdvbaseResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl TaxingAdvbaseConProv {
    pub(crate) fn new() -> TaxingAdvbaseConProv {
        TaxingAdvbaseConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(TaxingAdvbaseConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(TaxingAdvbaseConSpec, spec);

pub(crate) struct TaxingAdvpaymConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for TaxingAdvpaymConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptTaxingAdvpaym as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(TaxingAdvpaymConProv, provider, TaxingAdvpaymConSpec);

pub(crate) struct TaxingAdvpaymConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl TaxingAdvpaymConSpec {
    fn from_code(_code: ConceptCode) -> TaxingAdvpaymConSpec {
        let spec_path: Vec<i32> = vec![
            ExampleArticleConst::ArticleTaxingAdvbase as i32,
        ];
        TaxingAdvpaymConSpec {
            spec: ConceptSpec::new(_code, ConceptSpec::const_to_path_array(spec_path), Some(TaxingAdvpaymConSpec::concept_eval)),
        }
    }

    fn new() -> TaxingAdvpaymConSpec {
        TaxingAdvpaymConSpec::from_code(ConceptCode::get(TaxingAdvpaymConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = TaxingAdvpaymResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl TaxingAdvpaymConProv {
    pub(crate) fn new() -> TaxingAdvpaymConProv {
        TaxingAdvpaymConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(TaxingAdvpaymConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(TaxingAdvpaymConSpec, spec);

pub(crate) struct IncomeGrossConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for IncomeGrossConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptIncomeGross as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(IncomeGrossConProv, provider, IncomeGrossConSpec);

pub(crate) struct IncomeGrossConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl IncomeGrossConSpec {
    fn from_code(_code: ConceptCode) -> IncomeGrossConSpec {
        IncomeGrossConSpec {
            spec: ConceptSpec::new(_code, vec![], Some(IncomeGrossConSpec::concept_eval)),
        }
    }

    fn new() -> IncomeGrossConSpec {
        IncomeGrossConSpec::from_code(ConceptCode::get(IncomeGrossConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = IncomeGrossResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl IncomeGrossConProv {
    pub(crate) fn new() -> IncomeGrossConProv {
        IncomeGrossConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(IncomeGrossConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(IncomeGrossConSpec, spec);

pub(crate) struct IncomeNettoConProv {
    pub(crate) provider: ConceptSpecProvider,
}

impl IConceptSpecConst for IncomeNettoConProv {
    const CONCEPT_CODE: i32 = ExampleConceptConst::ConceptIncomeNetto as i32;
}

#[macro_use(crate::impl_concept_prov)]
crate::impl_concept_prov!(IncomeNettoConProv, provider, IncomeNettoConSpec);

pub(crate) struct IncomeNettoConSpec {
    spec: ConceptSpec,
}

#[allow(dead_code)]
impl IncomeNettoConSpec {
    fn from_code(_code: ConceptCode) -> IncomeNettoConSpec {
        let spec_path: Vec<i32> = vec![
            ExampleArticleConst::ArticleIncomeGross as i32,
            ExampleArticleConst::ArticleHealthInspaym as i32,
            ExampleArticleConst::ArticleSocialInspaym as i32,
            ExampleArticleConst::ArticleTaxingAdvpaym as i32,
        ];
        IncomeNettoConSpec {
            spec: ConceptSpec::new(_code, ConceptSpec::const_to_path_array(spec_path), Some(IncomeNettoConSpec::concept_eval)),
        }
    }

    fn new() -> IncomeNettoConSpec {
        IncomeNettoConSpec::from_code(ConceptCode::get(IncomeNettoConProv::CONCEPT_CODE))
    }
    fn concept_eval(target: ArcTermTarget, spec: Option<BoxArticleSpec>, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList {
        let results_values = IncomeNettoResult::new(target, spec);

        return vec!(Ok(Arc::new(results_values)));
    }
}

#[allow(dead_code)]
impl IncomeNettoConProv {
    pub(crate) fn new() -> IncomeNettoConProv {
        IncomeNettoConProv {
            provider: ConceptSpecProvider::new(ConceptCode::get(IncomeNettoConProv::CONCEPT_CODE)),
        }
    }
}

#[macro_use(crate::impl_concept_spec)]
crate::impl_concept_spec!(IncomeNettoConSpec, spec);

