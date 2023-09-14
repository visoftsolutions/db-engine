use crate::db_class::DbClassIdentifier;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassSimpleField {
    pub name: String,
    pub type_: String,
}

impl DbClassSimpleField {
    pub fn new(name: &str, type_: &str) -> DbClassField {
        DbClassSimpleField {
            name: name.to_string(),
            type_: type_.to_string(),
        }
        .into()
    }
}
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassLinkSingle{
    pub name: String,
    pub ident: DbClassIdentifier,
    pub prefetch: bool
}
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassLinkMultiple(DbClassIdentifier);

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum DbClassField {
    Simple(DbClassSimpleField),
    LinkSingle(DbClassLinkSingle),
    LinkMultiple(DbClassLinkMultiple),
}

impl From<DbClassSimpleField> for DbClassField {
    fn from(value: DbClassSimpleField) -> Self {
        DbClassField::Simple(value)
    }
}
