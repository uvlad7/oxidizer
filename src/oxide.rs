#[cfg(feature = "ext-pyo3")]
mod pyo3_oxide {
    use pyo3::prelude::PyModule;
    use pyo3::{Bound, PyResult};
    pub type OxyModule<'a> = Bound<'a, PyModule>;
    pub type OxyResult<T> = PyResult<T>;

    pub use oxidizer_macros::init_pyo3 as oxy_init;

    pub use oxidizer_macros::pyfunction as oxy_function;

    #[macro_export]
    macro_rules! wrap_oxy_pyfunction {
        ($function:path) => {{
            pyo3::prelude::wrap_pyfunction!($function)?
        }};
        ($function:path, $module:expr) => {{
            pyo3::prelude::wrap_pyfunction!($function, $module)?
        }};
    }
    pub use wrap_oxy_pyfunction as wrap_oxyfunction;

    // TODO: Create mappings for types, not just `paste`
    #[macro_export]
    macro_rules! oxy_pyexception {
        ($exception:expr, $message:expr) => {{
            paste::paste! {
                pyo3::exceptions::[<Py $exception>]::new_err($message)
            }
        }}
    }
    pub use oxy_pyexception as oxy_exception;

    #[macro_export]
    macro_rules! oxy_pysubmodule {
        ($module:ident, $name:literal) => {{
            paste::paste! {
                let submodule = pyo3::prelude::PyModule::new($module.py(), stringify!([<$name>]))?;
                $module.add_submodule(&submodule)?;
                submodule
            }
        }}
    }

    pub use oxy_pysubmodule as oxy_submodule;
}

#[cfg(feature = "ext-magnus")]
mod magnus_oxide {
    use magnus::method::Method;
    use magnus::{Error, Object, RModule};
    use std::marker::PhantomData;
    use std::ops::Deref;

    pub struct OxyModule<'a>(RModule, PhantomData<&'a ()>);

    impl Deref for OxyModule<'_> {
        type Target = RModule;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'a> From<RModule> for OxyModule<'a> {
        fn from(value: RModule) -> Self {
            OxyModule(value, PhantomData)
        }
    }

    impl OxyModule<'_> {
        pub fn add_function<M: Method>(&self, params: (&str, M)) -> Result<(), Error> {
            self.define_singleton_method(params.0, params.1)
        }
    }

    pub type OxyResult<T> = Result<T, Error>;

    pub use oxidizer_macros::init_magnus as oxy_init;
    pub use oxidizer_macros::rbfunction as oxy_function;

    #[macro_export]
    macro_rules! wrap_rbfunction {
        ($function:path) => {{
            use $function as wrapped_rbfunction;
            (wrapped_rbfunction::_OXY_NAME, wrapped_rbfunction::_OXY_WRAP)
        }};
        ($function:path, $ignored_module:expr) => {{
            use $function as wrapped_rbfunction;
            (wrapped_rbfunction::_OXY_NAME, wrapped_rbfunction::_OXY_WRAP)
        }};
    }
    pub use wrap_rbfunction as wrap_oxyfunction;

    // TODO: Create mappings for functions, not just `paste` with case convention
    #[macro_export]
    macro_rules! oxy_rbexception {
        ($exception:expr, $message:expr) => {{
            paste::paste! {
               magnus::Error::new(magnus::exception::[<$exception:snake>](), $message)
            }
        }}
    }
    pub use oxy_rbexception as oxy_exception;

    #[macro_export]
    macro_rules! oxy_rbsubmodule {
        ($module:ident, $name:literal) => {{
            paste::paste! {
                use magnus::Module;
                oxide::OxyModule::from($module.define_module(stringify!([<$name:camel>]))?)
            }
        }}
    }

    pub use oxy_rbsubmodule as oxy_submodule;
}

#[cfg(feature = "ext-pyo3")]
pub use pyo3_oxide::*;

#[cfg(feature = "ext-magnus")]
pub use magnus_oxide::*;
