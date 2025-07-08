use std::fmt;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[derive(Debug)]
pub enum Error {
    UnknownLedMode(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownLedMode(value) => write!(f,"Unknown Led Mode: {value}"),
        }
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            Error::UnknownLedMode(_) => PyValueError::new_err(err.to_string()),
        }
    }
}

