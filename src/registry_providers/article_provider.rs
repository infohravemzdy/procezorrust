use legalios::service::period::IPeriod;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::version_code::VersionCode;
use crate::service_types::article_define::{ArticleDefine, IArticleDefine};
use crate::registry_constants::concept_consts::ConceptConst;

pub(crate) trait IArticleSpecConst {
    const CONCEPT_CODE: i32;
}

pub(crate) trait IArticleSpec : IArticleDefine {
    fn get_sums(&self) -> Vec<ArticleCode>;
    fn get_defs(&self) -> ArticleDefine;
}

pub(crate) type BoxArticleSpec = Box<dyn IArticleSpec>;
pub(crate) type BoxArticleSpecList = Vec<BoxArticleSpec>;

#[derive(Debug, Clone)]
pub(crate) struct ArticleSpec {
    pub(crate) code: ArticleCode,
    pub(crate) role: ConceptCode,
    sums: Vec<ArticleCode>,
}

#[allow(dead_code)]
impl ArticleSpec {
    pub(crate) fn new(_code: ArticleCode, _role: ConceptCode, _sums: Vec<ArticleCode>) -> ArticleSpec {
        ArticleSpec {
            code: _code,
            role: _role,
            sums: _sums.to_vec(),
        }
    }

    pub(crate) fn const_to_sums_array(_codes: Vec<i32>) -> Vec<ArticleCode> {
        _codes.into_iter().map(|x| ArticleCode::get(x)).collect()
    }
    fn specs_to_sums_array(_codes: Vec<ArticleCode>) -> Vec<ArticleCode> {
        _codes.into_iter().map(|x| ArticleCode::get(x.get_value())).collect()
    }
}

impl IArticleDefine for ArticleSpec {
    fn get_code(&self) -> ArticleCode {
        self.code
    }
    fn get_role(&self) -> ConceptCode {
        self.role
    }
}

impl IArticleSpec for ArticleSpec {
    fn get_sums(&self) -> Vec<ArticleCode> {
        self.sums.to_vec()
    }

    fn get_defs(&self) -> ArticleDefine {
        ArticleDefine::get(self.code.get_value(), self.role.get_value())
    }
}

pub(crate) trait IArticleProvConst {
    const ARTICLE_CODE: i32;
}

pub(crate) trait IArticleSpecProvider {
    fn get_code(&self) -> ArticleCode;
    fn get_spec(&self, period: &dyn IPeriod, version: &VersionCode) -> BoxArticleSpec;
}

pub(crate) type BoxArticleSpecProvider = Box<dyn IArticleSpecProvider>;

pub(crate) struct ArticleSpecProvider {
    pub(crate) code: ArticleCode,
}

impl IArticleSpecProvider for ArticleSpecProvider {
    fn get_code(&self) -> ArticleCode {
        self.code
    }

    fn get_spec(&self, _period: &dyn IPeriod, _version: &VersionCode) -> Box<dyn IArticleSpec> {
        let concept = ConceptCode::get(ConceptConst::ConceptNotfound as i32);
        Box::new(ArticleSpec::new(self.code,concept, vec![]))
    }
}

impl ArticleSpecProvider {
    pub(crate) fn new(_code: ArticleCode) -> ArticleSpecProvider {
        ArticleSpecProvider {
            code: _code,
        }
    }
}

#[macro_export]
macro_rules! impl_article_spec {
    ($t:ident, $s:ident) => {
       #[allow(dead_code)]
       impl IArticleDefine for $t {
            fn get_code(&self) -> ArticleCode {
                self.$s.get_code()
            }
            fn get_role(&self) -> ConceptCode {
                self.$s.get_role()
            }
        }

        impl IArticleSpec for $t {
            fn get_sums(&self) -> Vec<ArticleCode> {
                self.$s.get_sums()
            }

            fn get_defs(&self) -> ArticleDefine {
                self.$s.get_defs()
            }
        }
    }
}

#[macro_export]
macro_rules! impl_article_prov {
    ($t:ident, $p:ident, $c:ident) => {
        #[allow(dead_code)]
        impl IArticleSpecProvider for $t {
            fn get_code(&self) -> ArticleCode {
                self.$p.get_code()
            }
            fn get_spec(&self, _period: &dyn IPeriod, _version: &VersionCode) -> BoxArticleSpec {
                Box::new($c::from_code(self.get_code()))
            }
        }
    }
}
