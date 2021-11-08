use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;

pub(crate) trait IArticleDefine {
    fn get_code(&self) -> ArticleCode;
    fn get_role(&self) -> ConceptCode;
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct ArticleDefine {
    code: ArticleCode,
    role: ConceptCode,
}

impl IArticleDefine for ArticleDefine {
    fn get_code(&self) -> ArticleCode {
        self.code
    }
    fn get_role(&self) -> ConceptCode {
        self.role
    }
}

impl ArticleDefine {
    pub(crate) fn new() -> ArticleDefine {
        ArticleDefine { code: ArticleCode::new(), role: ConceptCode::new() }
    }
    pub fn get(_code: i32, _role: i32) -> ArticleDefine {
        ArticleDefine { code: ArticleCode::get(_code), role: ConceptCode::get(_role) }
    }
    pub fn from_defs(_defs: &dyn IArticleDefine) -> ArticleDefine {
        ArticleDefine {
            code: ArticleCode::get(_defs.get_code().get_value()),
            role: ConceptCode::get(_defs.get_role().get_value()) }
    }
}