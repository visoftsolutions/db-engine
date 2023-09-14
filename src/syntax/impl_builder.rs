use proc_macro2::{Span, TokenStream, Ident};
use quote::{format_ident, quote};

use crate::{db_class::DbClass, syntax::string_to_iden, db_field::DbClassField};

use super::struct_builder::{StructSyntaxBuilder, Field};

impl DbClass {
    pub fn to_main_builder(&self) -> StructSyntaxBuilder {
        self.add_fields(self.id_builder(&self.ident.name))
    }

    pub fn to_id_builder(&self) -> StructSyntaxBuilder {
        self.id_builder(&self.ident.id_struct_name())
    }
    pub fn to_value_builder(&self) -> StructSyntaxBuilder{
        self.add_fields(StructSyntaxBuilder::new(&self.ident.value_struct_name()))
    }

    fn id_builder(&self, name: &str) -> StructSyntaxBuilder {
        let mut a = StructSyntaxBuilder::new(name);
        a.add_field(Field::with_decorators("id", "String", vec!["#[serde(deserialize_with = \"thing_to_string\")]"]));
        a
    }
    
    
    fn add_fields(&self, mut builder: StructSyntaxBuilder) -> StructSyntaxBuilder {
        for f in self.simple_fields() {
            builder.add_field(Field::new(&f.name, &f.type_));
        }
        for f in self.link_single_fields() {
            if f.prefetch {
                builder.add_field(Field::new(&f.name, &f.ident.name));
            } else {
                ()
            }
        }
        builder
    }
    pub fn to_impl_tokens(&self) -> TokenStream {
        let name_iden = string_to_iden(&self.ident.name);
        let db_iden_str = &self.ident.hash;
        let id_struct_iden = string_to_iden(&self.ident.id_struct_name());
        let value_struct_iden = string_to_iden(&self.ident.value_struct_name());
        
        let fields = self.simple_fields().into_iter().map(|f| format_ident!("{}", f.name)).collect::<Vec<_>>();

        quote! {
            impl #value_struct_iden {
                pub async fn db_create(&self, db: &Surreal<Client>) -> surrealdb::Result<Vec<#id_struct_iden>> {
                    db.create(#db_iden_str).content(self).await
                }

                pub async fn db_create_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Vec<#name_iden>> {
                    db.create(#db_iden_str).content(self).await
                }
            }

            impl #name_iden {
                pub async fn db_update(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<#id_struct_iden>> {
                    db.update((#db_iden_str, &self.id)).content(#value_struct_iden::from(self.clone())).await
                }
                pub async fn db_update_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<#name_iden>> {
                    db.update((#db_iden_str, &self.id)).content(#value_struct_iden::from(self.clone())).await
                }
            }

            impl #id_struct_iden {
                pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<#name_iden>> {
                    db.select((#db_iden_str, &self.id)).await
                }
            }

            impl From<#name_iden> for #value_struct_iden {
                fn from(value: #name_iden) -> Self {
                    #value_struct_iden {
                        #(#fields: value.#fields,)*
                    }
                }
            }

            impl From<#name_iden> for #id_struct_iden {
                fn from(value: #name_iden) -> Self {
                    #id_struct_iden {
                        id: value.id
                    }
                }
            }
            
        }
    }
}
