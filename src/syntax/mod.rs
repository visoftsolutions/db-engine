use proc_macro2::{Ident, Span};

pub mod impl_builder;
pub mod struct_builder;

fn string_to_iden(str: &str) -> Ident {
    syn::Ident::new(&str, Span::call_site())
}
