use crate::db_field::{DbClassLinkSingle, DbClassField};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum DbClassExtension {
    Custom(DbClassIdentifier),
    SimpleFill(DbClassLinkSingle)
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClass {
    ident: DbClassIdentifier,
    extends: Vec<DbClassExtension>,
    fields: Vec<DbClassField>
} 

impl DbClass {
    pub fn new(ident: DbClassIdentifier) -> Self {
        DbClass { ident, extends: vec![], fields: vec![] }
    }
    pub fn add_field(mut self, field: DbClassField) -> Self {
        self.fields.push(field);
        self
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassIdentifier{
    name: String,
    hash: String
}

impl DbClassIdentifier {
    pub fn new(name: String) -> Self {
        DbClassIdentifier::with_hash(name.clone(), sha256::digest(name))
    }
    pub fn with_hash(name: String, hash: String) -> Self {
        DbClassIdentifier { name, hash }
    }
}