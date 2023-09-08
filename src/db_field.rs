use crate::db_class::DbClassIdentifier;


#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassSimpleField {
    name: String,
    type_: String
}
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassLinkSingle(DbClassIdentifier);
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassLinkMultiple(DbClassIdentifier);

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum DbClassField {
    Simple(DbClassSimpleField),
    LinkSingle(DbClassLinkSingle),
    LinkMultiple(DbClassLinkMultiple)
}
