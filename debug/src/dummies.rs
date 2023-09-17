use proc_macro2::Ident;

pub fn debug(name: &Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    unimplemented!()
                }
        }
    }
}