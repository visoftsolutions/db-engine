use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    db_class::{DbClass, DbClassIdentifier},
    syntax::struct_builder::{Field, StructSyntaxBuilder},
};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbEnum {
    name: String,
    idents: Vec<DbClassIdentifier>,
    base: DbClass,
}

impl DbEnum {
    pub fn new(name: impl Into<String>, base: DbClass) -> Self {
        DbEnum {
            name: name.into(),
            idents: vec![base.ident.clone()],
            base: base.clone(),
        }
    }

    pub fn add_type(&mut self, ident: &DbClassIdentifier) {
        self.idents.push(ident.clone())
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let value_name = format_ident!("{}", self.base_name_inner());
        let variants: Vec<_> = self
            .idents
            .iter()
            .map(|i| {
                let name = format_ident!("{}", i.name);
                let hash = &i.hash;
                quote! {
                    #[serde(rename = #hash)]
                    #name(#name)
                }
            })
            .collect();
        let into_impls: Vec<_> = self
            .idents
            .iter()
            .map(|i| {
                let variant = format_ident!("{}", i.name);
                quote! {
                    impl Into<#name> for #variant {
                        fn into(self) -> #name {
                            #name::#variant(self)
                        }
                    }
                }
            })
            .collect();
        let variant_names: Vec<_> = self
            .idents
            .iter()
            .map(|i| format_ident!("{}", i.name))
            .collect();
        let value_struct = self.value_struct().to_tokens();
        quote! {
            #[derive(Debug, Serialize, Deserialize, Clone)]
            #[serde(tag = "type")]
            pub enum #name {
                #(#variants,)*
            }

            #(#into_impls)*

            #value_struct

            #[async_trait]
            impl DbExtend<#value_name> for #name {
                async fn db_extend(self, db: &Surreal<Client>) -> surrealdb::Result<#value_name>{
                    match self {
                        #(#name::#variant_names(v) => v.db_extend(db).await,)*
                    }
                }
            }
        }
    }

    fn value_struct(&self) -> StructSyntaxBuilder {
        let mut value_struct = StructSyntaxBuilder::new(self.base_name_inner(), "");
        for f in self.base.simple_fields() {
            value_struct.add_field(Field::new(f.name, f.type_));
        }
        value_struct
    }

    fn base_name_inner(&self) -> String {
        DbEnum::base_name(&self.name)
    }

    pub fn base_name(str: impl Into<String>) -> String {
        format!("{}Base", str.into())
    }
}
