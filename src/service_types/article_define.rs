use crate::service_types::article_code::ArticleCode;
use crate::service_types::article_seqs::ArticleSeqs;
use crate::service_types::article_term::ArticleTerm;
use crate::service_types::concept_code::ConceptCode;

pub trait IArticleDefine {
    fn get_code(&self) -> ArticleCode;
    fn get_seqs(&self) -> ArticleSeqs;
    fn get_role(&self) -> ConceptCode;
    fn get_term(&self) -> ArticleTerm;
}

#[derive(Debug, Copy, Clone)]
pub struct ArticleDefine {
    code: ArticleCode,
    seqs: ArticleSeqs,
    role: ConceptCode,
}

impl IArticleDefine for ArticleDefine {
    fn get_code(&self) -> ArticleCode {
        self.code
    }
    fn get_seqs(&self) -> ArticleSeqs { self.seqs }
    fn get_role(&self) -> ConceptCode {
        self.role
    }
    fn get_term(&self) -> ArticleTerm {
        ArticleTerm::get(self.code.value, self.seqs.value)
    }
}

impl ArticleDefine {
    pub fn new() -> ArticleDefine {
        ArticleDefine { code: ArticleCode::new(), seqs: ArticleSeqs::new(), role: ConceptCode::new() }
    }
    pub fn get(_code: i32, _seqs: i16, _role: i32) -> ArticleDefine {
        ArticleDefine { code: ArticleCode::get(_code), seqs: ArticleSeqs::get(_seqs), role: ConceptCode::get(_role) }
    }
    pub fn from_defs(_defs: &dyn IArticleDefine) -> ArticleDefine {
        ArticleDefine {
            code: ArticleCode::get(_defs.get_code().get_value()),
            seqs: ArticleSeqs::get(_defs.get_seqs().get_value()),
            role: ConceptCode::get(_defs.get_role().get_value()) }
    }
}