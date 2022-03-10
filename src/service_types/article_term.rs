use std::hash::{Hash, Hasher};
use crate::service_types::article_code::ArticleCode;
use crate::service_types::article_seqs::ArticleSeqs;

#[derive(Debug, Copy, Clone)]
pub(crate) struct ArticleTerm {
    pub(crate) code: ArticleCode,
    pub(crate) seqs: ArticleSeqs,
}

impl Hash for ArticleTerm {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seqs.hash(state);
        self.code.hash(state);
    }
}

impl PartialEq<Self> for ArticleTerm {
    fn eq(&self, other: &Self) -> bool {
        self.code.get_value()==other.code.get_value()
            && self.seqs.get_value()==other.seqs.get_value()
    }
}

impl Eq for ArticleTerm {
}

#[allow(dead_code)]
impl ArticleTerm {
    pub(crate) fn zero() -> ArticleTerm {
        ArticleTerm::new()
    }
    pub(crate) fn new() -> ArticleTerm {
        ArticleTerm { code: ArticleCode::new(), seqs: ArticleSeqs::new() }
    }
    pub(crate) fn get(_code: i32, _seqs: i16) -> ArticleTerm {
        ArticleTerm { code: ArticleCode::get(_code), seqs: ArticleSeqs::get(_seqs) }
    }
    pub(crate) fn from_term(_term: &ArticleTerm) -> ArticleTerm {
        ArticleTerm { code: ArticleCode::get(_term.get_code_value()), seqs: ArticleSeqs::get(_term.get_seqs_value()) }
    }
    pub(crate) fn get_code(&self) -> &ArticleCode { &self.code }
    pub(crate) fn get_code_value(&self) -> i32 {
        self.code.value
    }
    pub(crate) fn get_seqs(&self) -> &ArticleSeqs {
        &self.seqs
    }
    pub(crate) fn get_seqs_value(&self) -> i16 {
        self.seqs.value
    }
}