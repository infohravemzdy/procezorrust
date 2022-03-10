use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub(crate) struct VersionCode {
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
    pub(crate) fn zero() -> VersionCode {
        VersionCode::new()
    }
    pub(crate) fn new() -> VersionCode {
        VersionCode { value: 0 }
    }
    pub(crate) fn get(_value: i32) -> VersionCode {
        VersionCode { value: _value }
    }
    pub(crate) fn get_value(&self) -> i32 {
        self.value
    }
}