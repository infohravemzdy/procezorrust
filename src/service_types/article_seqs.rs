use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub struct ArticleSeqs {
    pub(crate) value: i16
}

impl Hash for ArticleSeqs {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq<Self> for ArticleSeqs {
    fn eq(&self, other: &Self) -> bool {
        self.get_value()==other.get_value()
    }
}

impl Eq for ArticleSeqs {
}

#[allow(dead_code)]
impl ArticleSeqs {
    pub fn zero() -> ArticleSeqs {
        ArticleSeqs::new()
    }
    pub fn new() -> ArticleSeqs {
        ArticleSeqs { value: 0 }
    }
    pub fn get(_value: i16) -> ArticleSeqs {
        ArticleSeqs { value: _value }
    }
    pub fn from_code(_code: &ArticleSeqs) -> ArticleSeqs {
        ArticleSeqs { value: _code.get_value() }
    }
    pub fn get_value(&self) -> i16 {
        self.value
    }
}