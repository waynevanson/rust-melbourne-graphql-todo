use async_graphql::*;
use database::Database;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

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
        let db = context.data::<Database>()?;
        let user_row = db
            .users
            .get(&id)
            .ok_or_else(|| Error::new("Could not retrieve User with id from database"))?;
        let user = User::from(user_row);
        Ok(user)
    }

    async fn users<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<User>> {
        let db = context.data::<Database>()?;
        let users = db
            .users
            .values()
            .map(|user| User::from(user))
            .collect::<Vec<_>>();
        Ok(users)
    }
}
