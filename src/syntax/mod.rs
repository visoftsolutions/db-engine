use proc_macro2::{Ident, Span};

pub mod struct_builder;
pub mod impl_builder;

fn string_to_iden(str: &str) -> Ident {
    syn::Ident::new(&str, Span::call_site())
}