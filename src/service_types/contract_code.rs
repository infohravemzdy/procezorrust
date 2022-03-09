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
    pub(crate) fn zero() -> ContractCode {
        ContractCode::new()
    }
    pub(crate) fn new() -> ContractCode {
        ContractCode { value: 0 }
    }
    pub(crate) fn get(_value: i16) -> ContractCode {
        ContractCode { value: _value }
    }
    pub(crate) fn get_value(&self) -> i16 {
        self.value
    }
    pub(crate) fn is_valid(&self) -> bool {
        self.value != 0
    }
}