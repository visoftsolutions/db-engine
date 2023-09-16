use futures::future::join_all;
use serde::{ser::Error, Deserialize, Deserializer, Serialize, Serializer};
use surrealdb::sql::Thing;
use surrealdb::{engine::remote::ws::Client, Surreal};
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
trait ClassHash {
    fn class_hash() -> String;
}
fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let original_value: Thing = Deserialize::deserialize(deserializer)?;
    Ok(original_value.id.to_string())
}
fn db_link_to_thing<S, T, U>(db_link: &DbLink<T, U>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<Thing>,
    T: Clone,
{
    let DbLink::Existing(e) = db_link else {
        return Err(Error::custom("Unable to serialize DbLink::New"))
    };
    let thing: Thing = e.clone().into();
    thing.serialize(serializer)
}
fn db_link_to_vec_thing<S, T, U>(
    db_link: &DbLink<Vec<T>, U>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<Thing>,
    T: Clone,
{
    let DbLink::Existing(e) = db_link else {
        return Err(Error::custom("Unable to serialize DbLink::New"))
    };
    let vec: Vec<Thing> = e.iter().map(|i| i.clone().into()).collect();
    vec.serialize(serializer)
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DbLink<S, T> {
    Existing(S),
    New(T),
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
    pub async fn db_create(mut self, db: &Surreal<Client>) -> surrealdb::Result<UserId> {
        let result: Vec<UserId> = db.create(UserId::class_hash()).content(self).await?;
        Ok(result.first().unwrap().clone())
    }
    pub async fn db_create_get(mut self, db: &Surreal<Client>) -> surrealdb::Result<User> {
        Ok(self.db_create(db).await?.db_get(&db).await?.unwrap())
    }
}
impl User {
    pub async fn db_update(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<UserId>> {
        db.update((UserId::class_hash(), &self.id))
            .content(ValueUser::from(self.clone()))
            .await
    }
}
impl UserId {
    pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<User>> {
        let Some(deserialized): Option<UserSerializer> = db
            .select((UserId::class_hash(), &self.id))
            .await? else { return Ok(None) };
        Ok(Some(User {
            id: self.id.clone(),
            name: deserialized.name,
            email: deserialized.email,
            age: deserialized.age,
        }))
    }
}
impl ClassHash for UserId {
    fn class_hash() -> String {
        "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d".to_string()
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
impl Into<Thing> for UserId {
    fn into(self) -> Thing {
        Thing::from((UserId::class_hash(), self.id))
    }
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
    pub doctor: Vec<User>,
    pub caretaker: Vec<UserId>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValuePet {
    pub name: String,
    #[serde(serialize_with = "db_link_to_thing")]
    pub owner: DbLink<UserId, ValueUser>,
    #[serde(serialize_with = "db_link_to_vec_thing")]
    pub doctor: DbLink<Vec<UserId>, Vec<ValueUser>>,
    #[serde(serialize_with = "db_link_to_vec_thing")]
    pub caretaker: DbLink<Vec<UserId>, Vec<ValueUser>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PetSerializer {
    pub name: String,
    pub owner: Thing,
    pub doctor: Vec<Thing>,
    pub caretaker: Vec<Thing>,
}
impl ValuePet {
    pub async fn db_create(mut self, db: &Surreal<Client>) -> surrealdb::Result<PetId> {
        if let DbLink::New(n) = self.owner {
            let result = n.db_create(db).await?;
            self.owner = DbLink::Existing(result);
        }
        if let DbLink::New(v) = self.caretaker {
            let futures = v.into_iter().map(|n| n.db_create(db)).collect::<Vec<_>>();
            let result = join_all(futures)
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
            self.caretaker = DbLink::Existing(result);
        }
        if let DbLink::New(v) = self.doctor {
            let futures = v.into_iter().map(|n| n.db_create(db)).collect::<Vec<_>>();
            let result = join_all(futures)
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
            self.doctor = DbLink::Existing(result);
        }
        let result: Vec<PetId> = db.create(PetId::class_hash()).content(self).await?;
        Ok(result.first().unwrap().clone())
    }
    pub async fn db_create_get(mut self, db: &Surreal<Client>) -> surrealdb::Result<Pet> {
        Ok(self.db_create(db).await?.db_get(&db).await?.unwrap())
    }
}
impl Pet {
    pub async fn db_update(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<PetId>> {
        db.update((PetId::class_hash(), &self.id))
            .content(ValuePet::from(self.clone()))
            .await
    }
}
impl PetId {
    pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<Pet>> {
        let Some(deserialized): Option<PetSerializer> = db
            .select((PetId::class_hash(), &self.id))
            .await? else { return Ok(None) };
        let Some(owner) = UserId {
            id: deserialized.owner.id.to_string(),
        }
            .db_get(db)
            .await? else { return Ok(None) };
        let Some(doctor) = join_all(
                deserialized
                    .doctor
                    .iter()
                    .map(|i| (|| async {
                        UserId { id: i.id.to_string() }.db_get(db).await
                    })())
                    .collect::<Vec<_>>(),
            )
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<Option<Vec<_>>>() else { return Ok(None) };
        let caretaker = deserialized
            .caretaker
            .iter()
            .map(|i| UserId {
                id: i.id.to_string(),
            })
            .collect();
        Ok(Some(Pet {
            id: self.id.clone(),
            owner,
            doctor,
            caretaker,
            name: deserialized.name,
        }))
    }
}
impl ClassHash for PetId {
    fn class_hash() -> String {
        "8f0d1b30eba5742686a57f8305a2b0455e7148c428fc4b36743a23b97982e6e0".to_string()
    }
}
impl From<Pet> for ValuePet {
    fn from(value: Pet) -> Self {
        ValuePet {
            name: value.name,
            owner: DbLink::Existing(UserId { id: value.owner.id }),
            doctor: DbLink::Existing(
                value
                    .doctor
                    .into_iter()
                    .map(|i| UserId { id: i.id })
                    .collect(),
            ),
            caretaker: DbLink::Existing(value.caretaker),
        }
    }
}
impl From<Pet> for PetId {
    fn from(value: Pet) -> Self {
        PetId { id: value.id }
    }
}
impl Into<Thing> for PetId {
    fn into(self) -> Thing {
        Thing::from((PetId::class_hash(), self.id))
    }
}
