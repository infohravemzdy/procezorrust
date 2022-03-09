use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub(crate) struct VariantCode {
    value: i16
}

impl Hash for VariantCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq<Self> for VariantCode {
    fn eq(&self, other: &Self) -> bool {
        self.get_value()==other.get_value()
    }
}

impl Eq for VariantCode {
}

#[allow(dead_code)]
impl VariantCode {
    pub(crate) fn zero() -> VariantCode {
        VariantCode::new()
    }
    pub(crate) fn new() -> VariantCode {
        VariantCode { value: 0 }
    }
    pub(crate) fn get(_value: i16) -> VariantCode {
        VariantCode { value: _value }
    }
    fn get_value(&self) -> i16 {
        self.value
    }
}