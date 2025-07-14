use std::fmt;

use pyo3::{prelude::*, types::PyType};

use crate::{PyEcErr};

fn hex_to_rgb(hex: u32) -> PyResult<(u8, u8, u8)> {
    if hex > 0xFFFFFF {
        return Err(PyEcErr::ColorToLarge(hex).into());
    }

    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    Ok((r, g, b))
}

#[pyclass(str)]
#[derive(Debug, Clone, Copy)]
pub struct Led {
    #[pyo3(get)]
    pub index: u8,
    #[pyo3(get)]
    pub color: (u8, u8, u8),
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
    pub fn from_hex(_cls: Bound<'_, PyType>, index: u8, hex: u32) -> PyResult<Self> {
        let color = hex_to_rgb(hex)?;
        Ok(
            Self {
                index,
                color,
                sync_color: color,
            }
        )
    }

    pub fn set_color_rgb(&mut self, r: u8, g:u8, b: u8) -> PyResult<()> {
        self.sync_color = (r, g, b);
        Ok(())
    }

    pub fn set_color_hex(&mut self, hex: u32) -> PyResult<()> {
        self.sync_color = hex_to_rgb(hex)?;
        Ok(())
    }

    pub fn sync_color(&mut self, ec: Bound<'_, crate::PyEc>) -> PyResult<()> {
        let mut ec_ref = ec.try_borrow_mut()?;
        self.sync(&mut ec_ref)
    }
}

impl Led {
    pub fn sync(&mut self, ec: &mut crate::PyEc) -> PyResult<()> {
        if self.index != 255 && self.color != self.sync_color {
                let (r, g, b) = self.sync_color;
                ec.led_set_color(self.index, r, g, b)?;
                self.color = self.sync_color;
        }
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

