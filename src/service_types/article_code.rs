use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub(crate) struct ArticleCode {
    pub(crate) value: i32
}

impl Hash for ArticleCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq<Self> for ArticleCode {
    fn eq(&self, other: &Self) -> bool {
        self.get_value()==other.get_value()
    }
}

impl Eq for ArticleCode {
}

impl ArticleCode {
    pub(crate) fn new() -> ArticleCode {
        ArticleCode { value: 0 }
    }
    pub(crate) fn get(_value: i32) -> ArticleCode {
        ArticleCode { value: _value }
    }
    pub(crate) fn from_code(_code: &ArticleCode) -> ArticleCode {
        ArticleCode { value: _code.get_value() }
    }
    pub(crate) fn get_value(&self) -> i32 {
        self.value
    }
}