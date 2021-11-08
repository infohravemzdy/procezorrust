use legalios::service::period::IPeriod;
use legalios::factories::bundle_props::IBundleProps;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::term_target::{ArcTermTarget};
use crate::service_types::version_code::VersionCode;
use crate::service_types::concept_define::IConceptDefine;
use crate::service_types::term_result::ResultArcTermResultList;

pub(crate) type ResultFunc = fn(target: ArcTermTarget, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps, _results: &ResultArcTermResultList) -> ResultArcTermResultList;

pub(crate) trait IConceptSpec: IConceptDefine {
    fn get_path(&self) -> Vec<ArticleCode>;
    fn get_result_delegate(&self) -> Option<ResultFunc>;
}

pub(crate) trait IConceptSpecConst {
    const CONCEPT_CODE: i32;
}

pub(crate) type BoxConceptSpec = Box<dyn IConceptSpec>;

#[derive(Clone)]
pub(crate) struct ConceptSpec {
    code: ConceptCode,
    path: Vec<ArticleCode>,
    result_delegate: Option<ResultFunc>,
}

impl IConceptDefine for ConceptSpec {
    fn get_code(&self) -> ConceptCode {
        self.code
    }
}

impl IConceptSpec for ConceptSpec {
    fn get_path(&self) -> Vec<ArticleCode> {
        self.path.to_vec()
    }

    fn get_result_delegate(&self) -> Option<ResultFunc> {
        self.result_delegate
    }
}

#[allow(dead_code)]
impl ConceptSpec {
    pub(crate) fn new(_code: ConceptCode, _path: Vec<ArticleCode>, _result: Option<ResultFunc>) -> ConceptSpec {
        ConceptSpec {
            code: _code,
            path: _path.to_vec(),
            result_delegate: _result,
        }
    }
    fn get_month_code(period: &dyn IPeriod) -> MonthCode {
        MonthCode::get(period.get_code())
    }

    pub(crate) fn const_to_path_array(_codes: Vec<i32>) -> Vec<ArticleCode> {
        _codes.into_iter().map(|x| ArticleCode::get(x)).collect()
    }
}

pub(crate) trait IConceptSpecProvider {
    fn get_code(&self) -> ConceptCode;
    fn get_spec(&self, period: &dyn IPeriod, version: &VersionCode) -> BoxConceptSpec;
}

pub(crate) struct ConceptSpecProvider {
    code: ConceptCode,
}

impl IConceptSpecProvider for ConceptSpecProvider {
    fn get_code(&self) -> ConceptCode {
        self.code
    }

    fn get_spec(&self, _period: &dyn IPeriod, _version: &VersionCode) -> Box<dyn IConceptSpec> {
        Box::new(ConceptSpec::new(self.code, vec![], None))
    }
}

impl ConceptSpecProvider {
    pub(crate) fn new(_code: ConceptCode) -> ConceptSpecProvider {
        ConceptSpecProvider {
            code: _code,
        }
    }
}

pub(crate) type BoxConceptSpecProvider = Box<dyn IConceptSpecProvider>;

#[macro_export]
macro_rules! impl_concept_spec {
    ($t:ident, $s:ident) => {
        #[allow(dead_code)]
        impl IConceptDefine for $t {
            fn get_code(&self) -> ConceptCode {
                self.$s.get_code()
            }
        }

        impl IConceptSpec for $t {
            fn get_path(&self) -> Vec<ArticleCode> {
                self.$s.get_path()
            }

            fn get_result_delegate(&self) -> Option<ResultFunc> {
                self.$s.get_result_delegate()
            }
        }
    }
}

#[macro_export]
macro_rules! impl_concept_prov {
    ($t:ident, $p:ident, $c:ident) => {
        #[allow(dead_code)]
        impl IConceptSpecProvider for $t {
            fn get_code(&self) -> ConceptCode {
                self.$p.get_code()
            }
            fn get_spec(&self, _period: &dyn IPeriod, _version: &VersionCode) -> BoxConceptSpec {
                Box::new($c::from_code(self.get_code()))
            }
        }
    }
}
