use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub(crate) struct ConceptCode {
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
    pub(crate) fn zero() -> ConceptCode {
        ConceptCode::new()
    }
    pub(crate) fn new() -> ConceptCode {
        ConceptCode { value: 0 }
    }
    pub(crate) fn get(_value: i32) -> ConceptCode {
        ConceptCode { value: _value }
    }
    pub(crate) fn get_value(&self) -> i32 {
        self.value
    }
}