use std::fmt;
#[derive(PartialEq, Clone, Debug)]
pub struct Address {
    data: AddressData,
}

impl Address {
    pub fn new_simple(address: u64) -> Address {
        Address {
            data: AddressData::Simple(address),
        }
    }

    pub fn new_indexed(address: Address, index: Address) -> Address {
        Address {
            data: AddressData::Indexed(address.as_u64(), index.as_u64()),
        }
    }

    pub fn as_u64(&self) -> u64 {
        match self.data {
            AddressData::Simple(address) => address,
            AddressData::Indexed(address, _index) => address,
        }
    }

    pub fn register_format(&self) -> String {
        match self.data {
            AddressData::Simple(address) => format!("r{}", address),
            AddressData::Indexed(address, index) => {
                format!("r{}[r{}]", address, index)
            }
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.register_format())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum AddressData {
    Simple(u64),
    Indexed(u64, u64),
}
