use proc_macro::TokenStream;

use syn::{DeriveInput, parse_macro_input};

mod derives;
mod dummies;

#[proc_macro_derive(CustomDebug,attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derives::derive_debug(&input).unwrap_or_else(|err| {
        let dummy = dummies::debug(&input.ident);
        to_compile_error(err, dummy)
    }).into()
}

fn to_compile_error(
    error: syn::Error,
    dummy: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let compile_errors = error.to_compile_error();
    quote::quote!(
        #dummy
        #compile_errors
    )
}
