use std::fmt;
use std::fmt::Write;

use pyo3::prelude::*;

use crate::{Led, PyEc, Syncable};

#[pyclass(str)]
#[derive(Debug)]
pub struct FrameBuffer {
    #[pyo3(get)]
    leds: Vec<Vec<Led>>,
    #[pyo3(get)]
    width: u8,
    #[pyo3(get)]
    height: u8,
    #[pyo3(get)]
    num_leds: usize,
}

#[pymethods]
impl FrameBuffer {
    #[new]
    pub fn new(led_map: Vec<Vec<u8>>) -> Self {
        let leds: Vec<Vec<Led>> = led_map
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .filter(|&idx| idx != 255)
                    .map(|idx| Led::new(idx,0,0,0))
                    .collect()
            })
            .collect();

        let height = leds.len() as u8;
        let width = leds.iter().map(|row| row.len()).max().unwrap_or(0) as u8;
        let num_leds = leds.iter().map(|row| row.len()).sum::<usize>();
        Self { leds, width, height, num_leds }
    }

    pub fn get(&self, row: usize, col: usize) -> PyResult<Option<Led>> {
        Ok(self.leds.get(row).and_then(|r| r.get(col)).cloned())
    }

    pub fn set(&mut self, row: usize, col: usize, r: u8, g: u8, b: u8) -> PyResult<()> {
        self.leds
            .get_mut(row)
            .and_then(|row_vec| row_vec.get_mut(col))
            .map(|led| led.set_color_rgb(r, g, b));
            Ok(())
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8) -> PyResult<()> {
        for row in &mut self.leds {
            for led in row.iter_mut() {
                led.set_color_rgb(r, g, b)?;
            }
        }
        Ok(())
    }

    pub fn clear(&mut self) -> PyResult<()> {
        self.fill(0, 0, 0)
    }

    #[getter]
    fn flat_leds(&self) -> Vec<Led> {
        self.leds.iter().flatten().cloned().collect()
    }

    pub fn render(&mut self, ec: Bound<'_, crate::PyEc>) -> PyResult<()> {
        let mut ec_ref = ec.try_borrow_mut()?;
        self.sync(&mut ec_ref)
    }
}


impl fmt::Display for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for row in &self.leds {
            for led in row {
                let (r, g, b) = led.color;
                // Append ANSI-colored square to buffer
                let _ = write!(
                    &mut buffer,
                    "\x1b[48;2;{r};{g};{b}m  \x1b[0m "
                );
            }
            buffer.push('\n');
        }

        f.write_str(&buffer)
    }

}

impl Syncable for FrameBuffer {
    fn sync(&mut self, ec: &mut PyEc) -> PyResult<()> {
        for row in &mut self.leds {
            for led in row {
                led.sync(ec)?;
            }
        }
        Ok(())
    }
}