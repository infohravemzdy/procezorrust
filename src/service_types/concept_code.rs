use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub struct ConceptCode {
    pub(crate) value: i32
}

impl Hash for ConceptCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq<Self> for ConceptCode {
    fn eq(&self, other: &Self) -> bool {
        self.get_value()==other.get_value()
    }
}

impl Eq for ConceptCode {
}

#[allow(dead_code)]
impl ConceptCode {
    pub fn zero() -> ConceptCode {
        ConceptCode::new()
    }
    pub fn new() -> ConceptCode {
        ConceptCode { value: 0 }
    }
    pub fn get(_value: i32) -> ConceptCode {
        ConceptCode { value: _value }
    }
    pub fn get_value(&self) -> i32 {
        self.value
    }
}