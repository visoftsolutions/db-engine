use crate::{
    db_field::{DbClassField, DbClassLinkSingle},
    syntax::struct_builder::StructSyntaxBuilder,
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
    pub fn id_struct_name(&self) -> String {
        self.ident.name.clone() + "Id"
    }
    pub fn create_struct_name(&self) -> String {
        "Create".to_string() + &self.ident.name
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
}

impl From<&DbClass> for StructSyntaxBuilder {
    fn from(value: &DbClass) -> Self {
        let mut s = StructSyntaxBuilder::new(&value.ident.name);
        s.add_field("id", "Thing");
        for field in &value.fields {
            if let DbClassField::Simple(f) = field {
                s.add_field(&f.name, &f.type_);
            }
        }
        s
    }
}
