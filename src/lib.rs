mod oxide;
use oxide::{wrap_oxyfunction, oxy_function};
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

/// A module implemented in Rust.
#[oxy_init]
fn init(module: &OxyModule<'_>) -> OxyResult<()> {
    module.add_function(wrap_oxyfunction!(hello, module))?;
    module.add_function(wrap_oxyfunction!(sum_as_string, module))?;
    // let sub = PyModule::new(m.py(), "oxy")?;
    // m.add_submodule(&sub)?;
    Ok(())
}
