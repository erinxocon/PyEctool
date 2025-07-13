use std::fmt;
use std::string;

use pyo3::prelude::*;
use pyo3::exceptions::{PyRuntimeError, PyUnicodeDecodeError};
use pyo3::types::PyType;

use hidapi::HidApi;

use ectool::{Ec, Error as EcError, Access, AccessHid};

use crate::PyEcErr;

fn ecerror_to_py_err(err: EcError) -> PyErr {
    PyRuntimeError::new_err(format!("EC error: {err:?}"))
}

fn hid_to_py_err(err: hidapi::HidError) -> PyErr {
    PyRuntimeError::new_err(format!("hidapi: {err}"))
}

fn string_to_py_err(err: string::FromUtf8Error) -> PyErr {
    PyUnicodeDecodeError::new_err(format!("{err}"))
}

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

#[pyclass(unsendable, str)]
pub struct PyEc {
    ec: Ec<Box<dyn Access>>,
    #[pyo3(get)]
    led_map: Vec<Vec<u8>>,
    #[pyo3(get)]
    board: String,
    #[pyo3(get)]
    version: String,
}

#[pymethods]
impl PyEc {
    #[new]
    pub fn new() -> PyResult<Self> {
        let ni = 255;
        let api = HidApi::new().map_err(hid_to_py_err)?;
        for info in api.device_list() {
            match (info.vendor_id(), info.product_id(), info.interface_number()) {
                // System76 Launch keyboards
                (0x3384, 0x0001..=0x000A, 1) => {
                    let device = info.open_device(&api).map_err(hid_to_py_err)?;
                    let access = AccessHid::new(device, 10, 100).map_err(ecerror_to_py_err)?;
                    let mut ec = unsafe { Ec::new(access).map_err(ecerror_to_py_err)? }.into_dyn();

                    //refactor this to set these per keyboard layout based on device info
                    let led_map = vec![
                        vec![69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83],
                        vec![68, 67, 66, 65, 64, 63, 62, 61, 60, 59, 58, 57, 56, 55, 54],
                        vec![39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53],
                        vec![38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, ni, 25],
                        vec![12, 11, 10,  9,  8,  7,  6,  5,  4,  3,  2,  1,  0, ni, ni],
                        vec![13, 14, 15, 16, 17, ni, 18, 19, 20, 21, ni, 22, 23, 24, ni],
                    ];

                    let data_size = unsafe { ec.access().data_size() };
                    let mut data = vec![0; data_size];


                    let board = {
                        let size = unsafe { ec.board(&mut data).map_err(ecerror_to_py_err)? };
                        data.truncate(size);
                        String::from_utf8(data.clone()).map_err(string_to_py_err)?
                    };

                    let version = {
                        let size = unsafe { ec.version(&mut data).map_err(ecerror_to_py_err)? };
                        data.truncate(size);
                        String::from_utf8(data).map_err(string_to_py_err)?
                    };


                    return Ok(Self { ec, led_map, board, version });
                }
                _ => {}
            }
        }

        Err(PyRuntimeError::new_err("No compatible EC HID device found"))
    }

    pub fn led_get_value(&mut self, index: u8) -> PyResult<(u8, u8)> {
        unsafe { self.ec.led_get_value(index).map_err(ecerror_to_py_err) }
    }

    pub fn led_set_value(&mut self, index: u8, value: u8) -> PyResult<()> {
        unsafe { self.ec.led_set_value(index, value).map_err(ecerror_to_py_err) }
    }

    pub fn led_get_mode(&mut self, layer: u8) -> PyResult<(LedMode, u8)> {
        let result = unsafe { self.ec.led_get_mode(layer).map_err(ecerror_to_py_err)? };
        let mode = LedMode::try_from(result.0)?;
        Ok((mode, result.1))
    }

    pub fn led_set_mode(&mut self, layer: u8, mode: LedMode, speed: u8) -> PyResult<()> {
        unsafe { self.ec.led_set_mode(layer, mode as u8, speed).map_err(ecerror_to_py_err) }
    }

    pub fn led_get_color(&mut self, index: u8) -> PyResult<(u8, u8, u8)> {
        unsafe { self.ec.led_get_color(index).map_err(ecerror_to_py_err) }
    }

    pub fn led_set_color(&mut self, index: u8, r: u8, g: u8, b: u8) -> PyResult<()> {
        unsafe { self.ec.led_set_color(index, r, g, b).map_err(ecerror_to_py_err) }
    }

    fn __repr__(&self) -> String {
        format!("<PyEc board={} version={}>", self.board, self.version)
    }
}

impl fmt::Display for PyEc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "System76 EC [{} / {}]", self.board, self.version)
    }
}

// put this somewhere else??
pub trait Syncable {
    fn sync(&mut self, ec: &mut PyEc) -> PyResult<()>;
}