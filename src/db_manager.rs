use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;

use crate::{db_class::DbClass, syntax::struct_builder::StructSyntaxBuilder};

pub struct DbManager {
    classes: HashSet<DbClass>,
}

impl DbManager {
    pub fn new() -> Self {
        DbManager {
            classes: HashSet::new(),
        }
    }

    pub fn add_class(mut self, class: DbClass) -> Self {
        self.classes.insert(class);
        self
    }
}

impl DbManager {
    pub fn to_tokens(self) -> TokenStream {
        let struct_tokens = self
            .classes
            .into_iter()
            .map(|c| StructSyntaxBuilder::from(c).to_tokens())
            .collect::<Vec<_>>();
        quote! {
            #(#struct_tokens)*
        }
    }
}
