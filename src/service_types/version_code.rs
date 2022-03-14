use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub struct VersionCode {
    value: i32
}

impl Hash for VersionCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq<Self> for VersionCode {
    fn eq(&self, other: &Self) -> bool {
        self.get_value()==other.get_value()
    }
}

impl Eq for VersionCode {
}

#[allow(dead_code)]
impl VersionCode {
    pub fn zero() -> VersionCode {
        VersionCode::new()
    }
    pub fn new() -> VersionCode {
        VersionCode { value: 0 }
    }
    pub fn get(_value: i32) -> VersionCode {
        VersionCode { value: _value }
    }
    pub fn get_value(&self) -> i32 {
        self.value
    }
}