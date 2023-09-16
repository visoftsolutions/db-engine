use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    db_class::{DbClass, DbClassExtension, DbClassIdentifier},
    db_enum::DbEnum,
};

pub struct DbManager {
    classes: Vec<DbClass>,
    enums: Vec<DbEnum>,
}

impl DbManager {
    pub fn new() -> Self {
        DbManager {
            classes: vec![],
            enums: vec![],
        }
    }

    pub fn add_class(&mut self, class: DbClass) -> DbClassIdentifier {
        let ident = class.ident.clone();
        self.classes.push(class);
        ident
    }

    pub fn add_enum(
        &mut self,
        name: impl Into<String>,
        base: &DbClassIdentifier,
        members: Vec<&DbClassIdentifier>,
    ) {
        let name: String = name.into();
        let base_class = self
            .classes
            .iter_mut()
            .find(|m| m.ident.hash == base.hash)
            .unwrap();
        base_class.extends_self(DbEnum::base_name(name.clone()));
        let mut enum_ = DbEnum::new(name, base_class.clone());
        for m in members {
            enum_.add_type(&m);
        }

        self.enums.push(enum_);
    }
    pub fn add_extension(
        &mut self,
        base: &DbClassIdentifier,
        name: impl Into<String>,
        ident: &DbClassIdentifier,
    ) {
        let name: String = name.into();
        let base_class = self
            .classes
            .iter()
            .find(|m| m.ident.hash == base.hash)
            .unwrap()
            .clone();
        let class = self
            .classes
            .iter_mut()
            .find(|m| m.ident.hash == ident.hash)
            .unwrap();
        let fields = class.simple_fields();
        let simple = base_class
            .simple_fields()
            .iter()
            .all(|f| fields.contains(f));
        class.extends(DbClassExtension(
            DbEnum::base_name(name.clone()),
            base_class,
            simple,
        ))
    }
}

impl DbManager {
    pub fn to_tokens(self) -> TokenStream {
        let struct_tokens = self
            .classes
            .into_iter()
            .map(|c| {
                let struct_ = c.to_main_builder().to_tokens();
                let id_struct = c.to_id_builder().to_tokens();
                let create_struct = c.to_value_builder().to_tokens();
                let serializer_struct = c.to_serializer_builder().to_tokens();
                let impl_ = c.to_impl_tokens();
                let impl_from = c.to_impl_from_tokens();
                quote! {
                    #id_struct
                    #struct_
                    #create_struct
                    #serializer_struct
                    #impl_
                    #impl_from
                }
            })
            .collect::<Vec<_>>();
        let enum_tokens = self.enums.iter().map(|e| e.to_tokens()).collect::<Vec<_>>();
        quote! {
            use async_trait::async_trait;
            use surrealdb::{Surreal, engine::remote::ws::Client};
            use serde::{Deserialize, Serialize, Deserializer, Serializer, ser::Error};
            use surrealdb::sql::Thing;
            use futures::future::join_all;

            #[derive(Debug, Deserialize)]
            struct Record {
                #[allow(dead_code)]
                id: Thing,
            }

            pub trait ClassHash{
                fn class_hash() -> String;
            }

            #[async_trait]
            pub trait DbExtend<T> {
                async fn db_extend(self, db: &Surreal<Client>) -> surrealdb::Result<T>;
            }

            #[async_trait]
            impl<T: Send> DbExtend<T> for T {
                async fn db_extend(self, _db: &Surreal<Client>) -> surrealdb::Result<T>{
                    Ok(self)
                }
            }

            fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
            where
                D: Deserializer<'de>,
            {
                let original_value: Thing = Deserialize::deserialize(deserializer)?;
                Ok(original_value.id.to_string())
            }
            fn db_link_to_thing<S, T, U>(db_link: &DbLink<T, U>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                T: Into<Thing>,
                T: Clone
            {
                let DbLink::Existing(e) = db_link else {return Err(Error::custom("Unable to serialize DbLink::New"))};
                let thing: Thing = e.clone().into();
                thing.serialize(serializer)
            }
            fn db_link_to_vec_thing<S, T, U>(db_link: &DbLink<Vec<T>, U>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                T: Into<Thing>,
                T: Clone
            {
                let DbLink::Existing(e) = db_link else {return Err(Error::custom("Unable to serialize DbLink::New"))};
                let vec: Vec<Thing> = e.iter().map(|i| i.clone().into()).collect();
                vec.serialize(serializer)
            }

            #[derive(Debug, Serialize, Deserialize, Clone)]
            pub enum DbLink<S, T> {
                Existing(S),
                New(T)
            }

            #(#struct_tokens)*
            #(#enum_tokens)*


        }
    }
}
