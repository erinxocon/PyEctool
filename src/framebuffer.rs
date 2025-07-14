use std::fmt;
use std::fmt::Write;
use std::thread;
use std::time::Duration;

use pyo3::prelude::*;

use crate::{Led, PyEc, PyEcErr};

#[pyclass(eq, rename_all = "SCREAMING_SNAKE_CASE", str)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RenderMode {
    TopBottom,
    BottomTop,
    LeftRight,
    RightLeft,
    OutsideIn,
    InsideOut,
    MiddleOut,
}

impl fmt::Display for RenderMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TopBottom => write!(f, "Top to Bottom"),
            Self::BottomTop => write!(f, "Bottom to Top"),
            Self::LeftRight => write!(f, "Left to Right"),
            Self::RightLeft => write!(f, "Right to Left"),
            Self::OutsideIn => write!(f, "Outside to In"),
            Self::InsideOut => write!(f, "Inside to Out"),
            Self::MiddleOut => write!(f, "Middle to Out"),
        }
    }
}


#[pyclass(str)]
#[derive(Debug)]
pub struct FrameBuffer {
    #[pyo3(get)]
    leds: Vec<Vec<Led>>,
    #[pyo3(get)]
    width: usize,
    #[pyo3(get)]
    height: usize,
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
                    // .filter(|&idx| idx != 255)
                    .map(|idx| Led::new(idx,0,0,0))
                    .collect()
            })
            .collect();

        let height = leds.len();
        let width = leds.iter().map(|row| row.len()).max().unwrap_or(0);
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

    pub fn fill_row(&mut self, row: usize, r: u8, g: u8, b: u8) -> PyResult<()> {
        if row >= self.height {
            return Err(PyEcErr::OutOfBounds(row, self.height).into());
        }
        for led in &mut self.leds[row] {
            led.set_color_rgb(r, g, b)?;
        }
        Ok(())
    }

    pub fn fill_column(&mut self, col: usize, r: u8, g: u8, b: u8) -> PyResult<()> {
        if col >= self.width {
            return Err(PyEcErr::OutOfBounds(col, self.width).into());
        }
        for row in &mut self.leds {
            if col < row.len() {
                row[col].set_color_rgb(r, g, b)?;
            }
        }
        Ok(())
    }

    pub fn clear(&mut self) -> PyResult<()> {
        self.fill(0, 0, 0)?;
        Ok(())
    }

    #[getter]
    fn flat_leds(&self) -> Vec<Led> {
        self.leds.iter().flatten().cloned().collect()
    }

    #[pyo3(signature = (ec, mode = None, reverse = None, sleep_dur = None))]
    pub fn render(&mut self, ec: Bound<'_, crate::PyEc>, mode: Option<RenderMode>, reverse: Option<bool>, sleep_dur: Option<u64>) -> PyResult<()> {
        let _mode = mode.unwrap_or(RenderMode::BottomTop);
        let _reverse = reverse.unwrap_or(false);
        let _sleep_dur = sleep_dur.unwrap_or(0);

        let mut ec_ref = ec.try_borrow_mut()?;
        self.sync(&mut ec_ref, _mode, _reverse, Duration::from_millis(_sleep_dur))
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

#[macro_export]
macro_rules! either_iter {
    ($iter:expr, $rev:expr) => {
        if $rev {
            ::either::Either::Right($iter.rev())
        } else {
            ::either::Either::Left($iter)
        }
    };
}

impl FrameBuffer {
    fn sync(&mut self, ec: &mut PyEc, mode: RenderMode, reverse: bool, sleep_dur: Duration) -> PyResult<()> {
        match mode {
            RenderMode::TopBottom => {
                for row in &mut self.leds {
                    let iter = either_iter!(row.iter_mut(), reverse);
                    for led in iter {
                        thread::sleep(sleep_dur);
                        led.sync(ec)?
                    }
                }
            },
            RenderMode::BottomTop => {
                for row in &mut self.leds.iter_mut().rev() {
                    let iter = either_iter!(row.iter_mut(), reverse);
                    for led in iter {
                        thread::sleep(sleep_dur);
                        led.sync(ec)?
                    }
                }
            },
            RenderMode::LeftRight => {
                for col in 0..self.width {
                    let iter = either_iter!(0..self.height, reverse);
                    for row in iter {
                        let mut led = self.leds[row][col];
                        thread::sleep(sleep_dur);
                        led.sync(ec)?
                    }
                }
            },
            RenderMode::RightLeft => {
                for col in (0..self.width).rev() {
                    let iter = either_iter!(0..self.height, reverse);
                    for row in iter {
                        let mut led = self.leds[row][col];
                        thread::sleep(sleep_dur);
                        led.sync(ec)?
                    }
                }
            },
            RenderMode::OutsideIn => {
                let mut top = 0;
                let mut bottom = self.height as isize - 1;
                let mut left = 0;
                let mut right = self.width as isize - 1;

                while top <= bottom && left <= right {
                    // left column (top to bottom)
                    for row in top..=bottom {
                        if let Some(led) = self.leds.get_mut(row as usize).and_then(|r| r.get_mut(left as usize)) {
                            led.sync(ec)?;
                            thread::sleep(sleep_dur);
                        }
                    }
                    left += 1;

                    // bottom row (left to right)
                    for col in left..=right {
                        if let Some(led) = self.leds.get_mut(bottom as usize).and_then(|r| r.get_mut(col as usize)) {
                            led.sync(ec)?;
                            thread::sleep(sleep_dur);
                        }
                    }
                    bottom -= 1;

                    if top <= bottom {
                        // right column (bottom to top)
                        for row in (top..=bottom).rev() {
                            if let Some(led) = self.leds.get_mut(row as usize).and_then(|r| r.get_mut(right as usize)) {
                                led.sync(ec)?;
                                thread::sleep(sleep_dur);
                            }
                        }
                        right -= 1;
                    }

                    if left <= right {
                        // top row (right to left)
                        for col in (left..=right).rev() {
                            if let Some(led) = self.leds.get_mut(top as usize).and_then(|r| r.get_mut(col as usize)) {
                                led.sync(ec)?;
                                thread::sleep(sleep_dur);
                            }
                        }
                        top += 1;
                    }
                }
            },
            RenderMode::InsideOut => todo!(),
            RenderMode::MiddleOut => todo!(),
        }

        Ok(())
    }

}