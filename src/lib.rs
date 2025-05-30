#[cfg(feature = "ext-magnus")]
use magnus::{function, prelude::*, Error, Ruby};

#[cfg(feature = "ext-magnus")]
fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

#[cfg(feature = "ext-magnus")]
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Oxidizer")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    Ok(())
}

#[cfg(feature = "ext-pyo3")]
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[cfg(feature = "ext-pyo3")]
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[cfg(feature = "ext-pyo3")]
#[pymodule]
fn oxidizer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}