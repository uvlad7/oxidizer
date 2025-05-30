#[cfg(feature = "ext-pyo3")]
mod pyo3_oxide {
    use pyo3::prelude::PyModule;
    use pyo3::{Bound, PyResult};
    pub type OxyModule<'a> = Bound<'a, PyModule>;
    pub type OxyResult<T> = PyResult<T>;

    pub use oxidizer_macros::init_pyo3 as oxy_init;
}

#[cfg(feature = "ext-magnus")]
mod magnus_oxide {
    use magnus::{Error, RModule};

    pub type OxyModule<'a> = RModule;

    pub type OxyResult<T> = Result<T, Error>;

    pub use magnus::init as oxy_init;
}

#[cfg(feature = "ext-pyo3")]
pub use pyo3_oxide::*;

#[cfg(feature = "ext-magnus")]
pub use magnus_oxide::*;
