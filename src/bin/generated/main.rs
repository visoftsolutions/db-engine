mod types;

use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use types::ValueUser;

use crate::types::{
    DbExtend, DbLink, PersonEnumBase, ValueCar, ValueGarage, ValueGuest, ValuePerson,
};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db_username = std::env::var("DB_USERNAME").unwrap();
    let db_password = std::env::var("DB_PASSWORD").unwrap();
    let db_ws = std::env::var("DB_WS").unwrap();

    // Connect to the server
    let db = Surreal::new::<Ws>(db_ws).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: db_username.as_str(),
        password: db_password.as_str(),
    })
    .await
    .unwrap();

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    let person = ValuePerson {
        age: 20,
        name: "Jan Kowalski".to_string(),
    }
    .db_create(&db)
    .await?;
    dbg!(&person);
    let user = ValueUser {
        PersonEnumBase: DbLink::Existing(person),
        email: "test@test.pl".to_string(),
    }
    .db_create_get(&db)
    .await?;
    dbg!(user);
    let guest = ValueGuest {
        PersonEnumBase: DbLink::New(ValuePerson {
            age: 20,
            name: "Mariusz Mariuszewski".to_string(),
        }),
        nick: "Marek1980".to_string(),
    }
    .db_create_get(&db)
    .await?;
    dbg!(&guest);
    let guest_person: PersonEnumBase = guest.clone().db_extend(&db).await?;
    dbg!(guest_person);

    let garage = ValueGarage {
        cars: DbLink::New(vec![
            ValueCar {
                owner: DbLink::Existing(guest.PersonEnumBase),
            },
            ValueCar {
                owner: DbLink::Existing(
                    ValueGuest {
                        nick: "Nested creation".to_string(),
                        PersonEnumBase: DbLink::New(ValuePerson {
                            name: "Even more nesting".to_string(),
                            age: 18,
                        }),
                    }
                    .db_create_get(&db)
                    .await?
                    .PersonEnumBase,
                ),
            },
            ValueCar {
                owner: DbLink::New(ValuePerson {
                    name: "Kacper Kacperski".to_string(),
                    age: 100,
                }),
            },
        ]),
    }
    .db_create_get(&db)
    .await?;
    dbg!(garage);

    Ok(())
}
