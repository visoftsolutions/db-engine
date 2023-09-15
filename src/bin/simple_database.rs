use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Name {
    first: String,
    last: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    title: String,
    name: Name,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Load envs
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
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Vec<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO".into(),
            name: Name {
                first: "Tobie".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: true,
        })
        .await?;
    dbg!(&created);

    // Update a person record with a specific id
    let updated: Option<Record> = db
        .update(&created[0].id)
        .merge(Responsibility { marketing: true })
        .await?;
    let updated = updated.unwrap();
    dbg!(&updated);

    // Select all people records
    let people: Vec<Record> = db.select("person").await?;
    dbg!(people);

    // Perform a custom advanced query
    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(groups);

    #[derive(Debug, Serialize, Deserialize)]
    struct Testing {
        name: String,
        person: (String, String),
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Testing2 {
        name: String,
        person: Thing,
    }

    let created: Vec<Record> = db
        .create("testing")
        .content(Testing {
            name: "Ola".into(),
            person: (updated.id.tb, updated.id.id.to_string()),
        })
        .await?;
    dbg!(&created);
    let full: Testing2 = db.select(&created[0].id).await?.unwrap();
    dbg!(&full);
    let person: Option<Person> = db.select(full.person).await?;
    dbg!(&person);

    Ok(())
}
