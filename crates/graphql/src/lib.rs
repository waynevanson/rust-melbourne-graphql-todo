mod user;

use async_graphql::*;
use database::Database;
use rand::random;
use std::sync::RwLock;
use user::{Name, Relationship, User};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn user<'ctx>(&self, context: &Context<'ctx>, id: u32) -> Result<User> {
        let db = context.data::<Pool>()?.0.read().unwrap();
        let user_row = db
            .persons
            .get(&id)
            .ok_or_else(|| Error::new("Could not retrieve User with id from database"))?;
        let user = User::from(user_row);
        Ok(user)
    }

    async fn users<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<User>> {
        let db = context.data::<Pool>()?.0.read().unwrap();
        let users = db
            .persons
            .values()
            .map(|user| User::from(user))
            .collect::<Vec<_>>();
        Ok(users)
    }
}

pub struct Pool(pub RwLock<Database>);

pub struct Mutation;

#[derive(InputObject)]
pub struct UserInput {
    first_name: String,
    last_name: String,
}

impl UserInput {
    fn into_user_by_ids<I>(self, ids: I) -> User
    where
        I: IntoIterator<Item = u32>,
    {
        let mut iter = ids.into_iter();
        let mut id = random();
        while iter.any(|id_| id == id_) {
            id = random();
        }

        User {
            id,
            name: Name {
                first: self.first_name,
                last: self.last_name,
            },
        }
    }

    fn into_user_rows_by_ids<I>(self, ids: I) -> database::Person
    where
        I: IntoIterator<Item = u32>,
    {
        let mut iter = ids.into_iter();
        let mut id = random();
        while iter.any(|id_| id == id_) {
            id = random();
        }

        database::Person {
            id,
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}

#[Object]
impl Mutation {
    async fn add_user<'ctx>(&self, context: &Context<'ctx>, user: UserInput) -> Result<User> {
        let db = context.data::<Pool>()?;
        let mut db = db.0.write().unwrap();
        let ids = db.persons.keys().copied();
        let user_row = user.into_user_rows_by_ids(ids);
        let user = User::from(&user_row);
        db.persons.insert(user_row.id, user_row);

        Ok(user)
    }
}

// facilities: sauna, steamroom, spa
// persons:

enum FacilityType {
    Sauna,
    SteamRoom,
    Spa,
}

struct Facility {
    _type: FacilityType,
    participants: Vec<Person>,
}

struct Person {
    facility: Option<Facility>,
    first_name: String,
    last_name: String,
}

// stream = person + enter or exit

enum Status {
    Entered,
    Exit,
}
