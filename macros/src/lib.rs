mod init {
    use convert_case::{Case, Casing};
    use proc_macro2::{Ident, Span, TokenStream};
    use quote::quote;
    use syn::{Error, ItemFn};

    fn crate_name(name: Option<String>) -> Result<String, Error> {
        match name {
            Some(v) => Ok(v),
            None => match std::env::var("CARGO_PKG_NAME") {
                Ok(v) => Ok(v),
                Err(_) => Err(Error::new(
                    Span::call_site(),
                    r#"missing (name = "...") attribute"#,
                )),
            },
        }
    }
    pub fn expand_pyo3(name: Option<String>, input: ItemFn) -> Result<TokenStream, Error> {
        let crate_name = crate_name(name)?;

        let extern_init_name = Ident::new(
            &format!("__oxidizer_init_{}", &crate_name),
            Span::call_site(),
        );
        let init_name = input.sig.ident.clone();
        // let init_params = &input.sig.inputs;
        // let init_args = init_params
        //     .iter()
        //     .map(|fn_arg| match fn_arg {
        //         FnArg::Typed(PatType { pat, .. }) => match &**pat {
        //             Pat::Ident(ident) => Ok(ident),
        //             _ => Err(Error::new(
        //                 Span::call_site(),
        //                 "argument pattern is not a simple ident",
        //             )),
        //         },
        //         FnArg::Receiver(_) => Err(Error::new(Span::call_site(), "argument is a receiver")),
        //     })
        //     .collect::<Result<Vec<_>, Error>>()?;
        // let init_ret = &input.sig.output;

        Ok(quote! {
            // FIXME
            use pyo3::types::PyModuleMethods;
            #input

            #[pyo3::pymodule(name=#crate_name)]
            fn #extern_init_name(module: &pyo3::Bound<'_, pyo3::prelude::PyModule>) -> pyo3::PyResult<()> {
                #init_name(module)
            }
        })
    }

    pub fn expand_magnus(name: Option<String>, input: ItemFn) -> Result<TokenStream, Error> {
        let crate_name = crate_name(name)?;
        let mod_name = &crate_name.to_case(Case::UpperCamel);
        let init_name = input.sig.ident.clone();
        let extern_init_name = Ident::new(
            &format!("__oxidizer_init_{}", &crate_name),
            Span::call_site(),
        );
        Ok(quote! {
            #input

            #[magnus::init(name=#crate_name)]
            fn #extern_init_name(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
                let module: oxide::OxyModule<'_> = ruby.define_module(#mod_name)?.into();
                #init_name(&module)
            }
        })
    }
}

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn init_pyo3(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut name = None;
    if !attrs.is_empty() {
        let attr_parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") {
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else {
                Err(meta.error("unsupported attribute"))
            }
        });
        parse_macro_input!(attrs with attr_parser);
    }
    init::expand_pyo3(name, parse_macro_input!(item))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn init_magnus(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut name = None;
    if !attrs.is_empty() {
        let attr_parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") {
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else {
                Err(meta.error("unsupported attribute"))
            }
        });
        parse_macro_input!(attrs with attr_parser);
    }
    init::expand_magnus(name, parse_macro_input!(item))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

mod function {
    use proc_macro2::TokenStream;
    use quote::quote;
    use std::str::FromStr;
    use syn::{Error, ItemFn};

    pub fn build_rb_function(name: Option<String>, input: ItemFn) -> Result<TokenStream, Error> {
        let fn_name = input.sig.ident.clone();
        let oxy_name = match name {
            Some(v) => v,
            None => fn_name.to_string(),
        };
        let oxy_arity = TokenStream::from_str(&input.sig.inputs.len().to_string())?;
        let oxy_args = TokenStream::from_str(&"magnus::Value, ".repeat(input.sig.inputs.len()))?;
        let hash = quote!(#);
        Ok(quote! {
            #input

            #hash[doc(hidden)]
            mod #fn_name {
                pub const _OXY_NAME: &str = #oxy_name;
                pub const _OXY_WRAP: unsafe extern "C" fn(#oxy_args magnus::Value) -> magnus::Value = { magnus::function!(super::#fn_name, #oxy_arity) };
            }
        })
    }

    pub fn build_py_function(
        name_opt: Option<String>,
        input: ItemFn,
    ) -> Result<TokenStream, Error> {
        let hash = quote!(#);
        let pyo3_annotation = match name_opt {
            Some(name) => {
                quote! {
                    #hash[pyo3(name = #name)]
                }
            }
            None => {
                quote! {}
            }
        };
        Ok(quote! {
            #hash[pyo3::prelude::pyfunction]
            #pyo3_annotation
            #input
        })
    }
}

#[proc_macro_attribute]
pub fn rbfunction(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut name = None;
    if !attrs.is_empty() {
        let attr_parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") {
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else {
                Err(meta.error("unsupported attribute"))
            }
        });
        parse_macro_input!(attrs with attr_parser);
    }

    function::build_rb_function(name, parse_macro_input!(input))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn pyfunction(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut name = None;
    if !attrs.is_empty() {
        let attr_parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") {
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else {
                Err(meta.error("unsupported attribute"))
            }
        });
        parse_macro_input!(attrs with attr_parser);
    }

    function::build_py_function(name, parse_macro_input!(input))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
