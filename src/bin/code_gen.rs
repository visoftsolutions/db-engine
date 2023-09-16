use std::fs::File;
use std::io::Write;

use db_engine::{
    db_class::DbClass, db_field::DbClassLinkMultiple as LnM, db_field::DbClassLinkSingle as LnS,
    db_field::DbClassSimpleField as SF, db_manager::DbManager,
};

fn main() {
    let mut mng = DbManager::new();
    let person = mng.add_class(
        DbClass::with_name("Person")
            .add_field(SF::new("name", "String"))
            .add_field(SF::new("age", "u16")),
    );
    let user = mng.add_class(DbClass::with_name("User").add_field(SF::new("email", "String")));
    let guest = mng.add_class(DbClass::with_name("Guest").add_field(SF::new("nick", "String")));
    let car = mng.add_class(DbClass::with_name("Car").add_field(LnS::new("owner", &person)));
    let _garage =
        mng.add_class(DbClass::with_name("Garage").add_field(LnM::new_prefetch("cars", &car)));
    mng.add_enum("PersonEnum", &person, vec![&user, &guest]);
    mng.add_extension(&person, "PersonEnum", &user);
    mng.add_extension(&person, "PersonEnum", &guest);

    let tokens = mng.to_tokens();
    let code = prettyplease::unparse(&syn::parse2(tokens).unwrap());
    let path = "src/bin/generated/types.rs";
    File::create(path)
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap();
    println!("Written to file: {}", path);
}
