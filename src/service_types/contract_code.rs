use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
pub(crate) struct ContractCode {
    value: i16
}

impl Hash for ContractCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq<Self> for ContractCode {
    fn eq(&self, other: &Self) -> bool {
        self.get_value()==other.get_value()
    }
}

impl Eq for ContractCode {
}

#[allow(dead_code)]
impl ContractCode {
    pub(crate) fn new() -> ContractCode {
        ContractCode { value: 0 }
    }
    pub(crate) fn get(_value: i16) -> ContractCode {
        ContractCode { value: _value }
    }
    fn get_value(&self) -> i16 {
        self.value
    }
}