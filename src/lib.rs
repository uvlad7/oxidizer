mod oxide;

use oxide::{oxy_exception, oxy_function, wrap_oxyfunction, oxy_submodule};
use oxide::{oxy_init, OxyModule, OxyResult};

/// Say hello
#[oxy_function]
fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

/// Formats the sum of two numbers as string.
#[oxy_function]
fn sum_as_string(a: usize, b: usize) -> OxyResult<String> {
    Ok((a + b).to_string())
}

/// Shows how to use exceptions
#[oxy_function(name = "odd")]
fn odd_is_odd(val: i64) -> OxyResult<bool> {
    if val % 2 == 0 {
        Err(oxy_exception!(RuntimeError, format!("{} is even", val)))
    } else {
        Ok(true)
    }
}

// Shows how to use pyo3/magnus directly
#[cfg(feature = "ext-pyo3")]
mod direct_access {
    use oxidizer_macros::pyfunction;
    use pyo3::prelude::*;
    use pyo3::types::PyString;
    use pyo3::{Bound, PyResult};
    #[pyfunction]
    pub fn inspect<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyString>> {
        value.repr()
    }
}

#[cfg(feature = "ext-magnus")]
mod direct_access {
    use super::*;
    use magnus::value::ReprValue;
    use magnus::{Error, RString, Value};

    #[oxy_function(name = "repr")]
    pub fn inspect(value: Value) -> Result<RString, Error> {
        Ok(RString::new(&value.inspect()))
    }
}

/// A module implemented in Rust.
#[oxy_init]
fn init(module: &OxyModule<'_>) -> OxyResult<()> {
    module.add_function(wrap_oxyfunction!(hello, module))?;
    module.add_function(wrap_oxyfunction!(sum_as_string, module))?;
    module.add_function(wrap_oxyfunction!(odd_is_odd, module))?;
    let submodule = oxy_submodule!(module, "snake_case");
    submodule.add_function(wrap_oxyfunction!(direct_access::inspect, module))?;
    let _ = oxy_submodule!(module, "CamelCase");
    Ok(())
}
