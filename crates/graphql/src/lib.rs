use std::sync::RwLock;

use async_graphql::*;
use database::Database;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(SimpleObject, Clone)]
pub struct Name {
    first: String,
    last: String,
}

impl From<&database::User> for Name {
    fn from(value: &database::User) -> Self {
        Self {
            first: value.first_name.to_string(),
            last: value.last_name.to_string(),
        }
    }
}

#[derive(SimpleObject)]
pub struct User {
    id: u32,
    name: Name,
}

#[derive(InputObject)]
pub struct UserInput {
    first_name: String,
    last_name: String,
}

impl From<&database::User> for User {
    fn from(value: &database::User) -> Self {
        Self {
            id: value.id,
            name: value.into(),
        }
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn user<'ctx>(&self, context: &Context<'ctx>, id: u32) -> Result<User> {
        let db = context.data::<Pool>()?.0.read().unwrap();
        let user_row = db
            .users
            .get(&id)
            .ok_or_else(|| Error::new("Could not retrieve User with id from database"))?;
        let user = User::from(user_row);
        Ok(user)
    }

    async fn users<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<User>> {
        let db = context.data::<Pool>()?.0.read().unwrap();
        let users = db
            .users
            .values()
            .map(|user| User::from(user))
            .collect::<Vec<_>>();
        Ok(users)
    }
}

pub struct Pool(pub RwLock<Database>);

pub struct Mutation;

#[Object]
impl Mutation {
    async fn add_user<'ctx>(&self, context: &Context<'ctx>, user: UserInput) -> Result<User> {
        let db = context.data::<Pool>()?;
        let mut db = db.0.write().unwrap();
        let id: u32 = rand::random();
        let user_row = database::User {
            id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
        };

        db.users.insert(id, user_row);

        let user = User {
            id,
            name: Name {
                first: user.first_name,
                last: user.last_name,
            },
        };

        Ok(user)
    }
}
