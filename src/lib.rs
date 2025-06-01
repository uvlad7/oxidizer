mod oxide;

use oxide::{oxy_exception, oxy_function, wrap_oxyfunction};
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

/// A module implemented in Rust.
#[oxy_init]
fn init(module: &OxyModule<'_>) -> OxyResult<()> {
    module.add_function(wrap_oxyfunction!(hello, module))?;
    module.add_function(wrap_oxyfunction!(sum_as_string, module))?;
    module.add_function(wrap_oxyfunction!(odd_is_odd, module))?;
    // let sub = PyModule::new(m.py(), "oxide")?;
    // m.add_submodule(&sub)?;
    Ok(())
}
