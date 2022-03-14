use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub struct PositionCode {
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
    pub fn zero() -> PositionCode {
        PositionCode::new()
    }
    pub fn new() -> PositionCode {
        PositionCode { value: 0 }
    }
    pub fn get(_value: i16) -> PositionCode {
        PositionCode { value: _value }
    }
    pub fn get_value(&self) -> i16 {
        self.value
    }
    pub fn is_valid(&self) -> bool {
        self.value != 0
    }
}