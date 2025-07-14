use std::fmt;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[derive(Debug)]
pub enum PyEcErr {
    UnknownLedMode(u8),
    ColorToLarge(u32),
    OutOfBounds(usize,usize),
}

impl fmt::Display for PyEcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownLedMode(mode) => write!(f,"Unknown Led Mode: {mode}"),
            Self::ColorToLarge(hex) => write!(f, "Color value must be a 24-bit RGB Color {hex} > 0xFFFFFF"),
            Self::OutOfBounds(value, target) => write!(f, "Value is out of bounds: {value} > {target}."),
        }
    }
}

impl From<PyEcErr> for PyErr {
    fn from(err: PyEcErr) -> PyErr {
        match err {
            PyEcErr::UnknownLedMode(_) |
            PyEcErr::ColorToLarge(_) |
            PyEcErr::OutOfBounds(_,_) => PyValueError::new_err(err.to_string())
        }
    }
}
