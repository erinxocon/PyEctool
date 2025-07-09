use std::fmt;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[derive(Debug)]
pub enum PyEcErr {
    UnknownLedMode(u8),
}

impl fmt::Display for PyEcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownLedMode(value) => write!(f,"Unknown Led Mode: {value}"),
        }
    }
}

impl From<PyEcErr> for PyErr {
    fn from(err: PyEcErr) -> PyErr {
        match err {
            PyEcErr::UnknownLedMode(_) => PyValueError::new_err(err.to_string()),
        }
    }
}

