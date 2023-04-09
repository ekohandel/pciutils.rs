use crate::bdf::BusDeviceFunction;
use std::fmt::Display;

pub struct Function {
    pub bdf: BusDeviceFunction,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bdf)
    }
}
