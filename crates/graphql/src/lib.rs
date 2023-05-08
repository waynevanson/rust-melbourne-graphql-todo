use async_graphql::*;
use database::{Database, GraphDB};
use std::sync::RwLock;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn person<'ctx>(&self, context: &Context<'ctx>, id: String) -> Result<Person> {
        let db = context.data::<GraphDB>()?;
        let person_raw = db.find_person(&id).ok_or_else(|| {
            Error::new(format!(
                "Could not find person with id {:?} in database",
                id
            ))
        })?;
        let person = Person::from(person_raw);
        Ok(person)
    }
}

pub struct Pool(pub RwLock<Database>);

#[derive(InputObject)]
pub struct UserInput {
    first_name: String,
    last_name: String,
}

#[derive(Debug, SimpleObject)]
struct Person {
    id: String,
    first_name: String,
    last_name: String,
}

impl From<database::Person> for Person {
    fn from(value: database::Person) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
        }
    }
}

// stream = person + enter or exit

enum Status {
    Entered,
    Exit,
}
