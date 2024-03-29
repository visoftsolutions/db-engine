use std::hash::Hash;
use std::hash::Hasher;

use crate::db_field::DbClassLinkSingle;
use crate::db_field::{DbClassField, DbClassLinkMultiple, DbClassSimpleField};

// #[derive(Eq, Hash, PartialEq, Debug, Clone)]
// pub enum DbClassExtension {
//     // Custom(DbClassIdentifier),
//     SimpleFill(String, DbClass),
// }

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassExtension(pub String, pub DbClass, pub bool);

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn extends(&mut self, ext: DbClassExtension) {
        if !ext.2 {
            self.fields
                .push(DbClassLinkSingle::new(&ext.0, &ext.1.ident));
        }
        self.extends.push(ext);
    }
    pub fn extends_self(&mut self, name: impl Into<String>) {
        let c = self.clone();
        self.extends(DbClassExtension(name.into(), c, true))
    }
    pub fn simple_fields(&self) -> Vec<DbClassSimpleField> {
        self.fields
            .iter()
            .filter_map(|f| {
                if let DbClassField::Simple(i) = f {
                    Some(i.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn link_single_fields(&self) -> Vec<DbClassLinkSingle> {
        self.fields
            .iter()
            .filter_map(|f| {
                if let DbClassField::LinkSingle(i) = f {
                    Some(i.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn link_multiple_fields(&self) -> Vec<DbClassLinkMultiple> {
        self.fields
            .iter()
            .filter_map(|f| {
                if let DbClassField::LinkMultiple(i) = f {
                    Some(i.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn serializer_struct_name(&self) -> String {
        self.name.clone() + "Serializer"
    }
}

impl Hash for DbClassIdentifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl Hash for DbClass {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ident.hash.hash(state);
    }
}
