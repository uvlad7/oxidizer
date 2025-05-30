mod oxide;

#[cfg(feature = "ext-magnus")]
use magnus::{function, prelude::*, Ruby};
use oxide::{oxy_init, OxyModule, OxyResult};

#[cfg(feature = "ext-magnus")]
fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

#[cfg(feature = "ext-magnus")]
#[oxy_init]
fn init(ruby: &Ruby) -> OxyResult<()> {
    let module: OxyModule = ruby.define_module("Oxidizer")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    Ok(())
}

#[cfg(feature = "ext-pyo3")]
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[cfg(feature = "ext-pyo3")]
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> OxyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[cfg(feature = "ext-pyo3")]
#[oxy_init]
fn init(module: &OxyModule<'_>) -> OxyResult<()> {
    // let sub = PyModule::new(m.py(), "oxy")?;
    // m.add_submodule(&sub)?;
    module.add_function(wrap_pyfunction!(sum_as_string, module)?)?;
    Ok(())
}
