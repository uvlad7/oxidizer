mod init {
    use proc_macro2::{Ident, Span, TokenStream};
    use quote::quote;
    use syn::{Error, FnArg, ItemFn, Pat, PatType};
    pub fn expand(name: Option<String>, input: ItemFn) -> Result<TokenStream, Error> {
        let crate_name = match name {
            Some(v) => v,
            None => match std::env::var("CARGO_PKG_NAME") {
                Ok(v) => v,
                Err(_) => {
                    return Err(Error::new(
                        Span::call_site(),
                        r#"missing (name = "...") attribute"#,
                    ))
                }
            },
        };

        let extern_init_name = Ident::new(&crate_name, Span::call_site());
        let init_name = input.sig.ident.clone();
        let init_params = &input.sig.inputs;
        let init_args = init_params
            .iter()
            .map(|fn_arg| match fn_arg {
                FnArg::Typed(PatType { pat, .. }) => match &**pat {
                    Pat::Ident(ident) => Ok(ident),
                    _ => Err(Error::new(
                        Span::call_site(),
                        "argument pattern is not a simple ident",
                    )),
                },
                FnArg::Receiver(_) => Err(Error::new(Span::call_site(), "argument is a receiver")),
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let init_ret = &input.sig.output;

        Ok(quote! {
            #input

            #[pymodule]
            fn #extern_init_name(#init_params) #init_ret {
                #init_name(#(#init_args)*)
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
    init::expand(name, parse_macro_input!(item))
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn init_magnus(attrs: TokenStream, item: TokenStream) -> TokenStream {
    unimplemented!()
}
