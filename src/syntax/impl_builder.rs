use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{db_class::DbClass, syntax::string_to_iden};

use super::struct_builder::{Field, StructSyntaxBuilder};

impl DbClass {
    pub fn to_main_builder(&self) -> StructSyntaxBuilder {
        let mut builder = self.id_builder(&self.ident.name);
        builder = self.add_simple_fields(builder);
        self.add_link_single_fields(builder)
    }

    pub fn to_id_builder(&self) -> StructSyntaxBuilder {
        self.id_builder(&self.ident.id_struct_name())
    }
    pub fn to_value_builder(&self) -> StructSyntaxBuilder {
        let mut builder = StructSyntaxBuilder::new(&self.ident.value_struct_name());
        builder = self.add_simple_fields(builder);
        self.add_link_single_fields_value(builder)
    }
    pub fn to_serializer_builder(&self) -> StructSyntaxBuilder {
        let mut builder = StructSyntaxBuilder::new(&self.ident.serializer_struct_name());
        builder = self.add_simple_fields(builder);
        builder = self.add_link_single_fields_serializer(builder);
        builder
    }

    fn id_builder(&self, name: &str) -> StructSyntaxBuilder {
        let mut a = StructSyntaxBuilder::new(name);
        a.add_field(Field::with_decorators(
            "id",
            "String",
            vec!["#[serde(deserialize_with = \"thing_to_string\")]"],
        ));
        a
    }
    fn add_simple_fields(&self, mut builder: StructSyntaxBuilder) -> StructSyntaxBuilder {
        for f in self.simple_fields() {
            builder.add_field(Field::new(&f.name, &f.type_));
        }
        builder
    }
    fn add_link_single_fields(&self, mut builder: StructSyntaxBuilder) -> StructSyntaxBuilder {
        for f in self.link_single_fields() {
            builder.add_field(Field::new(
                &f.name,
                if f.prefetch {
                    f.ident.name
                } else {
                    f.ident.id_struct_name()
                },
            ));
        }
        builder
    }
    fn add_link_single_fields_value(
        &self,
        mut builder: StructSyntaxBuilder,
    ) -> StructSyntaxBuilder {
        for f in self.link_single_fields() {
            builder.add_field(Field::with_decorators(
                &f.name,
                format!(
                    "DbLink<{}, {}>",
                    f.ident.id_struct_name(),
                    f.ident.value_struct_name()
                ),
                vec!["#[serde(serialize_with = \"db_link_to_thing\")]"],
            ));
        }
        builder
    }
    fn add_link_single_fields_serializer(
        &self,
        mut builder: StructSyntaxBuilder,
    ) -> StructSyntaxBuilder {
        for f in self.link_single_fields() {
            builder.add_field(Field::new(&f.name, "Thing"));
        }
        builder
    }
    pub fn to_impl_tokens(&self) -> TokenStream {
        let name_iden = string_to_iden(&self.ident.name);
        let db_iden_str = &self.ident.hash;
        let id_struct_iden = string_to_iden(&self.ident.id_struct_name());
        let value_struct_iden = string_to_iden(&self.ident.value_struct_name());
        let deserializer_struct_iden = string_to_iden(&self.ident.serializer_struct_name());

        let smp_fld = self
            .simple_fields()
            .into_iter()
            .map(|f| format_ident!("{}", f.name))
            .collect::<Vec<_>>();

        let (lnk_fetch, lnk): (Vec<_>, Vec<_>) = self
            .link_single_fields()
            .into_iter()
            .partition(|i| i.prefetch);
        let lnk_fetch_name = lnk_fetch
            .iter()
            .map(|f| format_ident!("{}", f.name))
            .collect::<Vec<_>>();
        let lnk_name = lnk
            .iter()
            .map(|f| format_ident!("{}", f.name))
            .collect::<Vec<_>>();
        let lnk_all_name = lnk_name
            .iter()
            .chain(lnk_fetch_name.iter())
            .cloned()
            .collect::<Vec<_>>();
        let lnk_fetch_types = &lnk_fetch
            .iter()
            .map(|f| format_ident!("{}", f.ident.id_struct_name()))
            .collect::<Vec<_>>();
        let lnk_types = &lnk
            .iter()
            .map(|f| format_ident!("{}", f.ident.id_struct_name()))
            .collect::<Vec<_>>();

        quote! {
            impl #value_struct_iden {
                pub async fn db_create(mut self, db: &Surreal<Client>) -> surrealdb::Result<#id_struct_iden> {
                    #(if let DbLink::New(n) = self.#lnk_all_name {
                        let result = n.db_create(db).await?;
                        self.#lnk_all_name = DbLink::Existing(result);
                    };)*
                    let result: Vec<#id_struct_iden> = db.create(#id_struct_iden::class_hash()).content(self).await?;
                    Ok(result.first().unwrap().clone())
                }

                pub async fn db_create_get(mut self, db: &Surreal<Client>) -> surrealdb::Result<#name_iden> {
                    Ok(self.db_create(db).await?.db_get(&db).await?.unwrap())
                }
            }

            impl #name_iden {
                pub async fn db_update(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<#id_struct_iden>> {
                    db.update((#id_struct_iden::class_hash(), &self.id)).content(#value_struct_iden::from(self.clone())).await
                }
                // pub async fn db_update_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<#name_iden>> {
                //     db.update((#db_iden_str, &self.id)).content(#value_struct_iden::from(self.clone())).await
                // }
            }

            impl #id_struct_iden {
                pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<#name_iden>> {
                    let Some(deserialized): Option<#deserializer_struct_iden> = db
                        .select((
                            #id_struct_iden::class_hash(),
                            &self.id,
                        ))
                        .await? else {return Ok(None)};
                    #(let Some(#lnk_fetch_name) = #lnk_fetch_types{id: deserialized.#lnk_fetch_name.id.to_string()}.db_get(db).await? else {return Ok(None)};)*
                    #(let #lnk_name = #lnk_types{id: deserialized.#lnk_name.id.to_string()};)*
                    Ok(Some(#name_iden{
                        id: self.id.clone(),
                        #(#lnk_fetch_name,)*
                        #(#lnk_name,)*
                        #(#smp_fld: deserialized.#smp_fld,)*
                    }))
                }
            }
            impl ClassHash for #id_struct_iden {
                fn class_hash() -> String {
                    #db_iden_str.to_string()
                }
            }
        }
    }
    pub fn to_impl_from_tokens(&self) -> TokenStream {
        let name_iden = string_to_iden(&self.ident.name);
        let id_struct_iden = string_to_iden(&self.ident.id_struct_name());
        let value_struct_iden = string_to_iden(&self.ident.value_struct_name());

        let simple_fields = self
            .simple_fields()
            .into_iter()
            .map(|f| format_ident!("{}", f.name))
            .collect::<Vec<_>>();

        let (lnk_fetch, lnk): (Vec<_>, Vec<_>) = self
            .link_single_fields()
            .into_iter()
            .partition(|i| i.prefetch);
        let lnk_fetch_name = lnk_fetch.iter().map(|f| format_ident!("{}", f.name));
        let lnk_name = lnk.iter().map(|f| format_ident!("{}", f.name));
        let lnk_fetch_types = &lnk_fetch
            .iter()
            .map(|f| format_ident!("{}", f.ident.id_struct_name()))
            .collect::<Vec<_>>();

        quote! {
            impl From<#name_iden> for #value_struct_iden {
                fn from(value: #name_iden) -> Self {
                    #value_struct_iden {
                        #(#simple_fields: value.#simple_fields,)*
                        #(#lnk_fetch_name: DbLink::Existing(#lnk_fetch_types{id: value.#lnk_fetch_name.id}), )*
                        #(#lnk_name: DbLink::Existing(value.#lnk_name), )*
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

            impl Into<Thing> for #id_struct_iden {
                fn into(self) -> Thing {
                    Thing::from((#id_struct_iden::class_hash(), self.id))
                }
            }

        }
    }
}
