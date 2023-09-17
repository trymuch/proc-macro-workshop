use proc_macro2::Ident;
use syn::{
    Data, DataStruct, DeriveInput,
    Expr, ExprLit, Field, Fields,
    GenericParam, Generics, Meta, MetaNameValue,
};
use syn::Lit::Str;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;

pub fn derive_debug(input: &DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = &input.ident;
    let generics = &input.generics;
    match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(ref fields), .. }) => {
            let fields = collect_fields(&fields.named)?;
            gen_for_struct(ident, generics, &fields)
        }
        _ => Err(syn::Error::new(input.span(), "`#[derive(CustomDebug)]` only supports structs"))
    }
}

fn collect_fields(fields: &Punctuated<Field, Comma>) -> Result<Vec<&Field>, syn::Error> {
    fields.iter().map(|field| {
        Ok(field)
    }).collect()
}

fn gen_for_struct(item_name: &Ident, item_generics: &Generics, fields: &[&Field]) -> Result<proc_macro2::TokenStream, syn::Error> {
    let fields_dummy = gen_for_fields(fields)?;
    let mut item_generics = item_generics.clone();
    bounds_generics(&mut item_generics, fields);
    let (impl_generics, type_generics, where_clause) =
        item_generics.split_for_impl();
    let tokens = quote::quote! {
                impl #impl_generics std::fmt::Debug for #item_name #type_generics #where_clause{
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_struct(stringify!(#item_name))
                            #fields_dummy
                            .finish()
                    }
                }
            };
    Ok(tokens)
}

fn bounds_generics(generics: &mut Generics, fields: &[&Field]) {
    let _ = generics.params.iter_mut().map(|g| {
        if let GenericParam::Type(t) = g {
            let type_ident = &t.ident;
            t.bounds.push(syn::parse_quote!(std::fmt::Debug));
        }
    }).collect::<Vec<_>>();
}

fn gen_for_fields(fields: &[&Field]) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut tokens = proc_macro2::TokenStream::new();
    for &field in fields {
        let ident = field.ident.as_ref().unwrap();
        let format_string = parse_field_format_string(field)?;
        if format_string.is_some() {
            tokens.extend(quote::quote! {
                .field(stringify!(#ident), &format_args!(#format_string, &self.#ident))
            });
        } else {
            tokens.extend(quote::quote! {
                .field(stringify!(#ident), &self.#ident)
            });
        }
    }
    Ok(tokens)
}

fn parse_field_format_string(field: &Field) -> Result<Option<String>, syn::Error> {
    for attr in &field.attrs {
        if let Meta::NameValue(MetaNameValue { ref path, ref value, .. }) = attr.meta {
            if path.segments.len() == 1 && path.segments.first().unwrap().ident == "debug" {
                if let Expr::Lit(ExprLit { lit: Str(lit_str), .. }) = value {
                    return Ok(Some(lit_str.value()));
                }
            }
        }
    }
    Ok(None)
}