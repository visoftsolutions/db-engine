use surrealdb::{Surreal, engine::remote::ws::Client};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub age: u16,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub age: u16,
}
impl CreateUser {
    pub async fn db_create(
        &self,
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Vec<UserId>> {
        let created: Vec<Record> = db
            .create("b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d")
            .content(self)
            .await?;
        Ok(created.into_iter().map(|c| UserId { id: c.id.id.to_string() }).collect())
    }
}
impl UserId {
    pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<User>> {
        db.select((
                "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d",
                &self.id,
            ))
            .await
    }
}
