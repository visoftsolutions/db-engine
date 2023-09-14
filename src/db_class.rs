use crate::{
    db_field::{DbClassField, DbClassLinkSingle, DbClassSimpleField},
    syntax::struct_builder::{StructSyntaxBuilder, Field},
};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum DbClassExtension {
    Custom(DbClassIdentifier),
    SimpleFill(DbClassLinkSingle),
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClass {
    pub(crate) ident: DbClassIdentifier,
    pub(crate) extends: Vec<DbClassExtension>,
    pub(crate) fields: Vec<DbClassField>,
}

impl DbClass {
    pub fn new(ident: DbClassIdentifier) -> Self {
        DbClass {
            ident,
            extends: vec![],
            fields: vec![],
        }
    }
    pub fn with_name(name: &str) -> Self {
        DbClass::new(DbClassIdentifier::new(name.to_string()))
    }
    pub fn add_field(mut self, field: DbClassField) -> Self {
        self.fields.push(field);
        self
    }
    pub fn simple_fields(&self) -> Vec<DbClassSimpleField> {
        self.fields.iter().filter_map(|f| {
            if let DbClassField::Simple(i) = f {
                Some(i.clone())
            } else {
                None
            }
        }).collect()
    }
    pub fn link_single_fields(&self) -> Vec<DbClassLinkSingle> {
        self.fields.iter().filter_map(|f| {
            if let DbClassField::LinkSingle(i) = f {
                Some(i.clone())
            } else {
                None
            }
        }).collect()
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassIdentifier {
    pub name: String,
    pub(crate) hash: String,
}

impl DbClassIdentifier {
    pub fn new(name: String) -> Self {
        DbClassIdentifier::with_hash(name.clone(), sha256::digest(name))
    }
    pub fn with_hash(name: String, hash: String) -> Self {
        DbClassIdentifier { name, hash }
    }
    pub fn id_struct_name(&self) -> String {
        self.name.clone() + "Id"
    }
    pub fn value_struct_name(&self) -> String {
        "Value".to_string() + &self.name
    }
}

