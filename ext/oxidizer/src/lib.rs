#[cfg(feature = "magnus")]
use magnus::{function, prelude::*, Error, Ruby};

#[cfg(feature = "magnus")]
fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

#[cfg(feature = "magnus")]
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Oxidizer")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    Ok(())
}
