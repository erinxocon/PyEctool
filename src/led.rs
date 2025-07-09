use pyo3::{prelude::*, types::PyType};

use std::fmt;
use crate::{PyEcErr};




fn hex_to_rgb(hex: u32) -> (u8, u8, u8) {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    (r, g, b)
}

#[pyclass(str)]
#[derive(Debug)]
struct Led {
    #[pyo3(get)]
    index: u8,
    #[pyo3(get)]
    color: (u8, u8, u8),
    #[pyo3(get)]
    sync_color: (u8, u8, u8),
}

#[pymethods]
impl Led {
    #[new]
    pub fn new(index: u8, r: u8, g: u8, b: u8) -> Self {
        Self {index, color: (r, g, b), sync_color: (r, g, b)}
    }

    #[classmethod]
    pub fn from_rgb(_cls: Bound<'_, PyType>, index: u8, color: (u8, u8, u8)) -> Self {
        Self { index, color, sync_color: color}
    }

    #[classmethod]
    pub fn from_hex(_cls: Bound<'_, PyType>, index: u8, hex: u32) -> Self {
        let color = hex_to_rgb(hex);
        Self {
            index,
            color,
            sync_color: color,
        }
    }

    pub fn set_color_rgb(&mut self, r: u8, g:u8, b: u8) -> PyResult<()> {
        self.sync_color = (r, g, b);
        Ok(())
    }

    pub fn set_color_hex(&mut self, hex: u32) -> PyResult<()> {
        self.sync_color = hex_to_rgb(hex);
        Ok(())
    }
}


impl fmt::Display for Led {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Red: {}, Green: {}, Blue: {}. Sync Status: {}.", 
            self.color.0,
            self.color.1,
            self.color.2,
            (self.color == self.sync_color)
        )
    }
}