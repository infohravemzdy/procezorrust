use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub struct ArticleCode {
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

#[allow(dead_code)]
impl ArticleCode {
    pub fn zero() -> ArticleCode {
        ArticleCode::new()
    }
    pub fn new() -> ArticleCode {
        ArticleCode { value: 0 }
    }
    pub fn get(_value: i32) -> ArticleCode {
        ArticleCode { value: _value }
    }
    pub fn from_code(_code: &ArticleCode) -> ArticleCode {
        ArticleCode { value: _code.get_value() }
    }
    pub fn get_value(&self) -> i32 {
        self.value
    }
}