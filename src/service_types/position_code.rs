use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub(crate) struct PositionCode {
    value: i16
}

impl Hash for PositionCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq<Self> for PositionCode {
    fn eq(&self, other: &Self) -> bool {
        self.get_value()==other.get_value()
    }
}

impl Eq for PositionCode {
}

#[allow(dead_code)]
impl PositionCode {
    pub(crate) fn new() -> PositionCode {
        PositionCode { value: 0 }
    }
    pub(crate) fn get(_value: i16) -> PositionCode {
        PositionCode { value: _value }
    }
    fn get_value(&self) -> i16 {
        self.value
    }
}