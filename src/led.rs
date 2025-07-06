use pyo3::prelude::*;

use std::fmt;
use crate::{Error};


#[pyclass(eq, eq_int, rename_all = "SCREAMING_SNAKE_CASE", str)]
#[derive(PartialEq, Debug)]
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
    type Error = Error;

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
            _ => Err(Error::UnknownLedMode),
        }
    }
}

#[pymethods]
impl LedMode {
    #[classmethod]
    pub fn from_int(_cls: &PyType, value: u8) -> PyResult<Self> {
        Self::try_from(value).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
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