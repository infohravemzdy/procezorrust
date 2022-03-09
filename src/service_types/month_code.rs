use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub(crate) struct MonthCode {
    value: i32
}

impl Hash for MonthCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq<Self> for MonthCode {
    fn eq(&self, other: &Self) -> bool {
        self.get_value()==other.get_value()
    }
}

impl Eq for MonthCode {
}

#[allow(dead_code)]
impl MonthCode {
    pub(crate) fn zero() -> MonthCode {
        MonthCode::new()
    }
    pub(crate) fn new() -> MonthCode {
        MonthCode { value: 0 }
    }
    pub(crate) fn get(_value: i32) -> MonthCode {
        MonthCode { value: _value }
    }
    fn get_value(&self) -> i32 {
        self.value
    }
}