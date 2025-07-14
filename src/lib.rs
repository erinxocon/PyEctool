pub use self::ec::{LedMode, PyEc};
mod ec;

pub use self::error::PyEcErr;
mod error;

pub use self::led::Led;
mod led;

pub use self::framebuffer::{FrameBuffer, RenderMode};
mod framebuffer;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pyectool(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<crate::PyEc>()?;
    m.add_class::<crate::LedMode>()?;
    m.add_class::<crate::Led>()?;
    m.add_class::<crate::FrameBuffer>()?;
    m.add_class::<crate::RenderMode>()?;
    Ok(())
}
