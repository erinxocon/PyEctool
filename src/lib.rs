pub use self::ec::PyEc;
mod ec;

// pub use self::led::LedMode;
// mod led;

// pub use self::error::Error;
// mod error;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pyectool(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<crate::ec::PyEc>()?;
    // m.add_class::<crate::led::LedMode>()?;
    Ok(())
}
