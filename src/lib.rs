pub use self::ec::PyEc;
pub use self::ec::LedMode;
pub use self::ec::Syncable;
mod ec;

pub use self::error::PyEcErr;
mod error;

pub use self::led::Led;
mod led;

pub use self::framebuffer::FrameBuffer;
mod framebuffer;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pyectool(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<crate::ec::PyEc>()?;
    m.add_class::<crate::ec::LedMode>()?;
    m.add_class::<crate::led::Led>()?;
    m.add_class::<crate::framebuffer::FrameBuffer>()?;
    Ok(())
}
