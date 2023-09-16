use async_trait::async_trait;
use futures::future::join_all;
use serde::{ser::Error, Deserialize, Deserializer, Serialize, Serializer};
use surrealdb::sql::Thing;
use surrealdb::{engine::remote::ws::Client, Surreal};
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
pub trait ClassHash {
    fn class_hash() -> String;
}
#[async_trait]
pub trait DbExtend<T> {
    async fn db_extend(self, db: &Surreal<Client>) -> surrealdb::Result<T>;
}
#[async_trait]
impl<T: Send> DbExtend<T> for T {
    async fn db_extend(self, _db: &Surreal<Client>) -> surrealdb::Result<T> {
        Ok(self)
    }
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
#[serde(tag = "type")]
#[serde(rename = "6007db63e18e532c7399975ed77d2e3900810aa75cad165b8d2e5d8b08085c3d")]
pub struct PersonId {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "6007db63e18e532c7399975ed77d2e3900810aa75cad165b8d2e5d8b08085c3d")]
pub struct Person {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub name: String,
    pub age: u16,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "6007db63e18e532c7399975ed77d2e3900810aa75cad165b8d2e5d8b08085c3d")]
pub struct ValuePerson {
    pub name: String,
    pub age: u16,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "6007db63e18e532c7399975ed77d2e3900810aa75cad165b8d2e5d8b08085c3d")]
pub struct PersonSerializer {
    pub name: String,
    pub age: u16,
}
impl ValuePerson {
    pub async fn db_create(mut self, db: &Surreal<Client>) -> surrealdb::Result<PersonId> {
        let result: Vec<PersonId> = db.create(PersonId::class_hash()).content(self).await?;
        Ok(result.first().unwrap().clone())
    }
    pub async fn db_create_get(mut self, db: &Surreal<Client>) -> surrealdb::Result<Person> {
        Ok(self.db_create(db).await?.db_get(&db).await?.unwrap())
    }
}
impl Person {
    pub async fn db_update(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<PersonId>> {
        db.update((PersonId::class_hash(), &self.id))
            .content(ValuePerson::from(self.clone()))
            .await
    }
}
impl PersonId {
    pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<Person>> {
        let Some(deserialized): Option<PersonSerializer> = db
            .select((PersonId::class_hash(), &self.id))
            .await? else { return Ok(None) };
        Ok(Some(Person {
            id: self.id.clone(),
            name: deserialized.name,
            age: deserialized.age,
        }))
    }
}
impl ClassHash for PersonId {
    fn class_hash() -> String {
        "6007db63e18e532c7399975ed77d2e3900810aa75cad165b8d2e5d8b08085c3d".to_string()
    }
}
impl From<Person> for ValuePerson {
    fn from(value: Person) -> Self {
        ValuePerson {
            name: value.name,
            age: value.age,
        }
    }
}
impl From<Person> for PersonId {
    fn from(value: Person) -> Self {
        PersonId { id: value.id }
    }
}
impl Into<Thing> for PersonId {
    fn into(self) -> Thing {
        Thing::from((PersonId::class_hash(), self.id))
    }
}
#[async_trait]
impl DbExtend<PersonEnumBase> for Person {
    async fn db_extend(self, db: &Surreal<Client>) -> surrealdb::Result<PersonEnumBase> {
        Ok(PersonEnumBase {
            name: self.name,
            age: self.age,
        })
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d")]
pub struct UserId {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d")]
pub struct User {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub email: String,
    pub PersonEnumBase: PersonId,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d")]
pub struct ValueUser {
    pub email: String,
    #[serde(serialize_with = "db_link_to_thing")]
    pub PersonEnumBase: DbLink<PersonId, ValuePerson>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d")]
pub struct UserSerializer {
    pub email: String,
    pub PersonEnumBase: Thing,
}
impl ValueUser {
    pub async fn db_create(mut self, db: &Surreal<Client>) -> surrealdb::Result<UserId> {
        if let DbLink::New(n) = self.PersonEnumBase {
            let result = n.db_create(db).await?;
            self.PersonEnumBase = DbLink::Existing(result);
        }
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
        let PersonEnumBase = PersonId {
            id: deserialized.PersonEnumBase.id.to_string(),
        };
        Ok(Some(User {
            id: self.id.clone(),
            PersonEnumBase,
            email: deserialized.email,
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
            email: value.email,
            PersonEnumBase: DbLink::Existing(value.PersonEnumBase),
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
#[async_trait]
impl DbExtend<PersonEnumBase> for User {
    async fn db_extend(self, db: &Surreal<Client>) -> surrealdb::Result<PersonEnumBase> {
        let base = self.PersonEnumBase.db_get(db).await?.unwrap();
        Ok(PersonEnumBase {
            name: base.name,
            age: base.age,
        })
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "5ed8944a85a9763fd315852f448cb7de36c5e928e13b3be427f98f7dc455f141")]
pub struct GuestId {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "5ed8944a85a9763fd315852f448cb7de36c5e928e13b3be427f98f7dc455f141")]
pub struct Guest {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub nick: String,
    pub PersonEnumBase: PersonId,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "5ed8944a85a9763fd315852f448cb7de36c5e928e13b3be427f98f7dc455f141")]
pub struct ValueGuest {
    pub nick: String,
    #[serde(serialize_with = "db_link_to_thing")]
    pub PersonEnumBase: DbLink<PersonId, ValuePerson>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "5ed8944a85a9763fd315852f448cb7de36c5e928e13b3be427f98f7dc455f141")]
pub struct GuestSerializer {
    pub nick: String,
    pub PersonEnumBase: Thing,
}
impl ValueGuest {
    pub async fn db_create(mut self, db: &Surreal<Client>) -> surrealdb::Result<GuestId> {
        if let DbLink::New(n) = self.PersonEnumBase {
            let result = n.db_create(db).await?;
            self.PersonEnumBase = DbLink::Existing(result);
        }
        let result: Vec<GuestId> = db.create(GuestId::class_hash()).content(self).await?;
        Ok(result.first().unwrap().clone())
    }
    pub async fn db_create_get(mut self, db: &Surreal<Client>) -> surrealdb::Result<Guest> {
        Ok(self.db_create(db).await?.db_get(&db).await?.unwrap())
    }
}
impl Guest {
    pub async fn db_update(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<GuestId>> {
        db.update((GuestId::class_hash(), &self.id))
            .content(ValueGuest::from(self.clone()))
            .await
    }
}
impl GuestId {
    pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<Guest>> {
        let Some(deserialized): Option<GuestSerializer> = db
            .select((GuestId::class_hash(), &self.id))
            .await? else { return Ok(None) };
        let PersonEnumBase = PersonId {
            id: deserialized.PersonEnumBase.id.to_string(),
        };
        Ok(Some(Guest {
            id: self.id.clone(),
            PersonEnumBase,
            nick: deserialized.nick,
        }))
    }
}
impl ClassHash for GuestId {
    fn class_hash() -> String {
        "5ed8944a85a9763fd315852f448cb7de36c5e928e13b3be427f98f7dc455f141".to_string()
    }
}
impl From<Guest> for ValueGuest {
    fn from(value: Guest) -> Self {
        ValueGuest {
            nick: value.nick,
            PersonEnumBase: DbLink::Existing(value.PersonEnumBase),
        }
    }
}
impl From<Guest> for GuestId {
    fn from(value: Guest) -> Self {
        GuestId { id: value.id }
    }
}
impl Into<Thing> for GuestId {
    fn into(self) -> Thing {
        Thing::from((GuestId::class_hash(), self.id))
    }
}
#[async_trait]
impl DbExtend<PersonEnumBase> for Guest {
    async fn db_extend(self, db: &Surreal<Client>) -> surrealdb::Result<PersonEnumBase> {
        let base = self.PersonEnumBase.db_get(db).await?.unwrap();
        Ok(PersonEnumBase {
            name: base.name,
            age: base.age,
        })
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "a5cdf07dbbc15892dcb64f1553ebb474330393705ce470b22f0194f582234371")]
pub struct CarId {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "a5cdf07dbbc15892dcb64f1553ebb474330393705ce470b22f0194f582234371")]
pub struct Car {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub owner: PersonId,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "a5cdf07dbbc15892dcb64f1553ebb474330393705ce470b22f0194f582234371")]
pub struct ValueCar {
    #[serde(serialize_with = "db_link_to_thing")]
    pub owner: DbLink<PersonId, ValuePerson>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "a5cdf07dbbc15892dcb64f1553ebb474330393705ce470b22f0194f582234371")]
pub struct CarSerializer {
    pub owner: Thing,
}
impl ValueCar {
    pub async fn db_create(mut self, db: &Surreal<Client>) -> surrealdb::Result<CarId> {
        if let DbLink::New(n) = self.owner {
            let result = n.db_create(db).await?;
            self.owner = DbLink::Existing(result);
        }
        let result: Vec<CarId> = db.create(CarId::class_hash()).content(self).await?;
        Ok(result.first().unwrap().clone())
    }
    pub async fn db_create_get(mut self, db: &Surreal<Client>) -> surrealdb::Result<Car> {
        Ok(self.db_create(db).await?.db_get(&db).await?.unwrap())
    }
}
impl Car {
    pub async fn db_update(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<CarId>> {
        db.update((CarId::class_hash(), &self.id))
            .content(ValueCar::from(self.clone()))
            .await
    }
}
impl CarId {
    pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<Car>> {
        let Some(deserialized): Option<CarSerializer> = db
            .select((CarId::class_hash(), &self.id))
            .await? else { return Ok(None) };
        let owner = PersonId {
            id: deserialized.owner.id.to_string(),
        };
        Ok(Some(Car {
            id: self.id.clone(),
            owner,
        }))
    }
}
impl ClassHash for CarId {
    fn class_hash() -> String {
        "a5cdf07dbbc15892dcb64f1553ebb474330393705ce470b22f0194f582234371".to_string()
    }
}
impl From<Car> for ValueCar {
    fn from(value: Car) -> Self {
        ValueCar {
            owner: DbLink::Existing(value.owner),
        }
    }
}
impl From<Car> for CarId {
    fn from(value: Car) -> Self {
        CarId { id: value.id }
    }
}
impl Into<Thing> for CarId {
    fn into(self) -> Thing {
        Thing::from((CarId::class_hash(), self.id))
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "b15f6253519634d53233fa8fe692697f192a15708d5ab1fe8013276a1111592d")]
pub struct GarageId {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "b15f6253519634d53233fa8fe692697f192a15708d5ab1fe8013276a1111592d")]
pub struct Garage {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub cars: Vec<Car>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "b15f6253519634d53233fa8fe692697f192a15708d5ab1fe8013276a1111592d")]
pub struct ValueGarage {
    #[serde(serialize_with = "db_link_to_vec_thing")]
    pub cars: DbLink<Vec<CarId>, Vec<ValueCar>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "b15f6253519634d53233fa8fe692697f192a15708d5ab1fe8013276a1111592d")]
pub struct GarageSerializer {
    pub cars: Vec<Thing>,
}
impl ValueGarage {
    pub async fn db_create(mut self, db: &Surreal<Client>) -> surrealdb::Result<GarageId> {
        if let DbLink::New(v) = self.cars {
            let futures = v.into_iter().map(|n| n.db_create(db)).collect::<Vec<_>>();
            let result = join_all(futures)
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
            self.cars = DbLink::Existing(result);
        }
        let result: Vec<GarageId> = db.create(GarageId::class_hash()).content(self).await?;
        Ok(result.first().unwrap().clone())
    }
    pub async fn db_create_get(mut self, db: &Surreal<Client>) -> surrealdb::Result<Garage> {
        Ok(self.db_create(db).await?.db_get(&db).await?.unwrap())
    }
}
impl Garage {
    pub async fn db_update(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<GarageId>> {
        db.update((GarageId::class_hash(), &self.id))
            .content(ValueGarage::from(self.clone()))
            .await
    }
}
impl GarageId {
    pub async fn db_get(&self, db: &Surreal<Client>) -> surrealdb::Result<Option<Garage>> {
        let Some(deserialized): Option<GarageSerializer> = db
            .select((GarageId::class_hash(), &self.id))
            .await? else { return Ok(None) };
        let Some(cars) = join_all(
                deserialized
                    .cars
                    .iter()
                    .map(|i| (|| async {
                        CarId { id: i.id.to_string() }.db_get(db).await
                    })())
                    .collect::<Vec<_>>(),
            )
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<Option<Vec<_>>>() else { return Ok(None) };
        Ok(Some(Garage {
            id: self.id.clone(),
            cars,
        }))
    }
}
impl ClassHash for GarageId {
    fn class_hash() -> String {
        "b15f6253519634d53233fa8fe692697f192a15708d5ab1fe8013276a1111592d".to_string()
    }
}
impl From<Garage> for ValueGarage {
    fn from(value: Garage) -> Self {
        ValueGarage {
            cars: DbLink::Existing(value.cars.into_iter().map(|i| CarId { id: i.id }).collect()),
        }
    }
}
impl From<Garage> for GarageId {
    fn from(value: Garage) -> Self {
        GarageId { id: value.id }
    }
}
impl Into<Thing> for GarageId {
    fn into(self) -> Thing {
        Thing::from((GarageId::class_hash(), self.id))
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum PersonEnum {
    #[serde(rename = "6007db63e18e532c7399975ed77d2e3900810aa75cad165b8d2e5d8b08085c3d")]
    Person(Person),
    #[serde(rename = "b512d97e7cbf97c273e4db073bbb547aa65a84589227f8f3d9e4a72b9372a24d")]
    User(User),
    #[serde(rename = "5ed8944a85a9763fd315852f448cb7de36c5e928e13b3be427f98f7dc455f141")]
    Guest(Guest),
}
impl Into<PersonEnum> for Person {
    fn into(self) -> PersonEnum {
        PersonEnum::Person(self)
    }
}
impl Into<PersonEnum> for User {
    fn into(self) -> PersonEnum {
        PersonEnum::User(self)
    }
}
impl Into<PersonEnum> for Guest {
    fn into(self) -> PersonEnum {
        PersonEnum::Guest(self)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename = "")]
pub struct PersonEnumBase {
    pub name: String,
    pub age: u16,
}
#[async_trait]
impl DbExtend<PersonEnumBase> for PersonEnum {
    async fn db_extend(self, db: &Surreal<Client>) -> surrealdb::Result<PersonEnumBase> {
        match self {
            PersonEnum::Person(v) => v.db_extend(db).await,
            PersonEnum::User(v) => v.db_extend(db).await,
            PersonEnum::Guest(v) => v.db_extend(db).await,
        }
    }
}
