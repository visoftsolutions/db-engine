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
            .map(|c| {
                let struct_ = StructSyntaxBuilder::from(&c).to_tokens();
                let id_struct = c.to_id_struct_tokens();
                let create_struct = c.to_create_struct_tokens();
                let impl_ = c.to_impl_tokens();
                quote! {
                    #id_struct
                    #struct_
                    #create_struct
                    #impl_
                }
            })
            .collect::<Vec<_>>();
        quote! {
            use surrealdb::{Surreal, engine::remote::ws::Client};
            use serde::{Deserialize, Serialize, Deserializer};
            use surrealdb::sql::Thing;

            #[derive(Debug, Deserialize)]
            struct Record {
                #[allow(dead_code)]
                id: Thing,
            }

            fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
            where
                D: Deserializer<'de>,
            {
                let original_value: Thing = Deserialize::deserialize(deserializer)?;
                Ok(original_value.id.to_string())
            }

            #(#struct_tokens)*
        }
    }
}
