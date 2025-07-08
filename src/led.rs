use pyo3::{prelude::*, types::PyType};

use std::fmt;
use crate::{Error as PyEcErr};


#[pyclass(eq, eq_int, rename_all = "SCREAMING_SNAKE_CASE", str)]
#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum LedMode {
    SolidColor = 0,
    PerKey,
    CycleAll,
    CycleLeftRight,
    CycleUpDown,
    CycleOutIn,
    CycleOutInDual,
    RainbowMovingChevron,
    CyclePinwheel,
    CycleSpiral,
    Raindrops,
    Splash,
    Multisplash,
    ActiveKeys,
    Disabled,
    Last,
}

impl TryFrom<u8> for LedMode {
    type Error = PyEcErr;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SolidColor),
            1 => Ok(Self::PerKey),
            2 => Ok(Self::CycleAll),
            3 => Ok(Self::CycleLeftRight),
            4 => Ok(Self::CycleUpDown),
            5 => Ok(Self::CycleOutIn),
            6 => Ok(Self::CycleOutInDual),
            7 => Ok(Self::RainbowMovingChevron),
            8 => Ok(Self::CyclePinwheel),
            9 => Ok(Self::CycleSpiral),
            10 => Ok(Self::Raindrops),
            11 => Ok(Self::Splash),
            12 => Ok(Self::Multisplash),
            13 => Ok(Self::ActiveKeys),
            14 => Ok(Self::Disabled),
            15 => Ok(Self::Last),
            other => Err(PyEcErr::UnknownLedMode(other)),
        }
    }
}

#[pymethods]
impl LedMode {
    #[classmethod]
    pub fn from_int(_cls: Bound<'_, PyType>, value: u8) -> PyResult<Self> {
        Ok(Self::try_from(value)?)
    }
}

impl fmt::Display for LedMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SolidColor => write!(f, "Solid Color"),
            Self::PerKey => write!(f, "Per Key"),
            Self::CycleAll => write!(f, "Cycle All"),
            Self::CycleLeftRight => write!(f, "Cycle Left to Right"),
            Self::CycleUpDown => write!(f, "Cycle Up to Down"),
            Self::CycleOutIn => write!(f, "Cycle Out to In"),
            Self::CycleOutInDual => write!(f, "Cycle Out to In Dual"),
            Self::RainbowMovingChevron => write!(f, "Rainbow Chevron"),
            Self::CyclePinwheel => write!(f, "Pinwheel"),
            Self::CycleSpiral => write!(f, "Spiral"),
            Self::Raindrops => write!(f, "Raindrops"),
            Self::Splash => write!(f, "Splash"),
            Self::Multisplash => write!(f, "Multisplash"),
            Self::ActiveKeys => write!(f, "Active Keys"),
            Self::Disabled => write!(f, "Disabled"),
            Self::Last => write!(f, "Last"),
        }
    }
}

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