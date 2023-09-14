use surrealdb::{Surreal, engine::remote::ws::Client};
use serde::{Deserialize, Serialize, Deserializer};
use surrealdb::sql::Thing;
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let original_value: Thing = Deserialize::deserialize(deserializer)?;
    Ok(original_value.id.to_string())
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserId {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u16,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValueUser {
    pub name: String,
    pub email: String,
    pub age: u16,
}
impl ValueUser {
    pub async fn db_create(
        &self,
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Vec<UserId>> {
        db.create("b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d")
            .content(self)
            .await
    }
    pub async fn db_create_get(
        &self,
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Vec<User>> {
        db.create("b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d")
            .content(self)
            .await
    }
}
impl User {
    pub async fn db_update(
        &self,
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Option<UserId>> {
        db.update((
                "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d",
                &self.id,
            ))
            .content(ValueUser::from(self.clone()))
            .await
    }
    pub async fn db_update_get(
        &self,
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Option<User>> {
        db.update((
                "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d",
                &self.id,
            ))
            .content(ValueUser::from(self.clone()))
            .await
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
impl From<User> for ValueUser {
    fn from(value: User) -> Self {
        ValueUser {
            name: value.name,
            email: value.email,
            age: value.age,
        }
    }
}
impl From<User> for UserId {
    fn from(value: User) -> Self {
        UserId { id: value.id }
    }
}
