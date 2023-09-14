use proc_macro2::{Span, TokenStream, Ident};
use quote::{format_ident, quote};

use crate::{db_class::DbClass, syntax::string_to_iden, db_field::DbClassField};

use super::struct_builder::StructSyntaxBuilder;

impl DbClass {
    pub fn to_id_struct_tokens(&self) -> TokenStream {
        StructSyntaxBuilder::new(&self.id_struct_name()).add_field("id", "String").to_tokens()
    }
    pub fn to_create_struct_tokens(&self) -> TokenStream {
        let mut s = StructSyntaxBuilder::new(&self.create_struct_name());

        for field in &self.fields {
            if let DbClassField::Simple(f) = field {
                s.add_field(&f.name, &f.type_);
            }
        }
        s.to_tokens()
    }
    pub fn to_impl_tokens(&self) -> TokenStream {
        let name_iden = string_to_iden(&self.ident.name);
        let db_iden_str = &self.ident.hash;
        let id_struct_iden = string_to_iden(&self.id_struct_name());
        let create_struct_iden = string_to_iden(&self.create_struct_name());
        
        quote! {
            impl #create_struct_iden {
                pub async fn db_create(&self, db: &Surreal<Client>) -> surrealdb::Result<Vec<#id_struct_iden>> {
                    let created: Vec<Record> = db
                        .create(#db_iden_str)
                        .content(self)
                        .await?;
                    Ok(created.into_iter().map(|c| #id_struct_iden{id: c.id.id.to_string()}).collect())
                }
            }

            impl #id_struct_iden {
                pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<#name_iden>> {
                    db.select((#db_iden_str, &self.id)).await
                }
            }
        }
    }
}