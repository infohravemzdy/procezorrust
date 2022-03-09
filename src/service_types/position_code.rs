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
    pub(crate) fn zero() -> PositionCode {
        PositionCode::new()
    }
    pub(crate) fn new() -> PositionCode {
        PositionCode { value: 0 }
    }
    pub(crate) fn get(_value: i16) -> PositionCode {
        PositionCode { value: _value }
    }
    pub(crate) fn get_value(&self) -> i16 {
        self.value
    }
    pub(crate) fn is_valid(&self) -> bool {
        self.value != 0
    }
}