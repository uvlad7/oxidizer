#[cfg(feature = "ext-pyo3")]
mod pyo3_oxide {
    use pyo3::prelude::PyModule;
    use pyo3::{Bound, PyResult};
    pub type OxyModule<'a> = Bound<'a, PyModule>;
    pub type OxyResult<T> = PyResult<T>;

    pub use oxidizer_macros::init_pyo3 as oxy_init;

    pub use pyo3::prelude::pyfunction as oxy_function;

    #[macro_export]
    macro_rules! wrap_pyfunction {
        ($function:path) => {{
            pyo3::prelude::wrap_pyfunction!($function)?
        }};
        ($function:path, $module:expr) => {{
            pyo3::prelude::wrap_pyfunction!($function, $module)?
        }};
    }
    pub use wrap_pyfunction as wrap_oxyfunction;
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
            paste::paste! {
                ([<_OXY_NAME_ $function>], [<_OXY_WRAP_ $function>])
            }
        }};
        ($function:path, $ignored_module:expr) => {{
            paste::paste! {
                ([<_OXY_NAME_ $function>], [<_OXY_WRAP_ $function>])
            }
        }};
    }
    pub use wrap_rbfunction as wrap_oxyfunction;
}

#[cfg(feature = "ext-pyo3")]
pub use pyo3_oxide::*;

#[cfg(feature = "ext-magnus")]
pub use magnus_oxide::*;
