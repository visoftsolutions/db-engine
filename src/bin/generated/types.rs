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
pub enum DbLink<S, T> {
    Existing(S),
    New(T),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PetId {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pet {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub name: String,
    pub owner: User,
    pub doctor: UserId,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValuePet {
    pub name: String,
    pub owner: DbLink<UserId, User>,
    pub doctor: DbLink<UserId, User>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PetSerializer {
    pub name: String,
    pub owner: Thing,
    pub doctor: Thing,
}
impl ValuePet {
    pub async fn db_create(
        &self,
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Vec<PetId>> {
        db.create("8f0d1b30eba5742686a57f8305a2b0455e7148c428fc4b36743a23b97982e6e0")
            .content(self)
            .await
    }
}
impl Pet {
    pub async fn db_update(
        &self,
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Option<PetId>> {
        db.update((
                "8f0d1b30eba5742686a57f8305a2b0455e7148c428fc4b36743a23b97982e6e0",
                &self.id,
            ))
            .content(ValuePet::from(self.clone()))
            .await
    }
    pub async fn db_update_get(
        &self,
        db: &Surreal<Client>,
    ) -> surrealdb::Result<Option<Pet>> {
        db.update((
                "8f0d1b30eba5742686a57f8305a2b0455e7148c428fc4b36743a23b97982e6e0",
                &self.id,
            ))
            .content(ValuePet::from(self.clone()))
            .await
    }
}
impl PetId {
    pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<Pet>> {
        let deserialized: PetSerializer = db
            .select((
                "8f0d1b30eba5742686a57f8305a2b0455e7148c428fc4b36743a23b97982e6e0",
                &self.id,
            ))
            .await;
    }
}
impl From<Pet> for ValuePet {
    fn from(value: Pet) -> Self {
        ValuePet {
            name: value.name,
            owner: DbLink::Existing(UserId { id: value.owner.id }),
            doctor: DbLink::Existing(value.doctor),
        }
    }
}
impl From<Pet> for PetId {
    fn from(value: Pet) -> Self {
        PetId { id: value.id }
    }
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSerializer {
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
        let deserialized: UserSerializer = db
            .select((
                "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d",
                &self.id,
            ))
            .await;
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
