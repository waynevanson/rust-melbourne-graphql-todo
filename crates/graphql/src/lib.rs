use std::time::Duration;

use async_graphql::{futures_util::Stream, *};
use database::GraphDB;
use rand::random;
use tokio_stream::StreamExt;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn person<'ctx>(&self, context: &Context<'ctx>, id: String) -> Result<Person> {
        context
            .data::<GraphDB>()?
            .find_person(&id)
            .map(Person::from)
            .ok_or_else(|| {
                Error::new(format!(
                    "Could not find person with id {:?} in database",
                    id
                ))
            })
    }

    async fn persons<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<Person>> {
        Ok(context
            .data::<GraphDB>()?
            .find_persons()
            .into_iter()
            .map(Person::from)
            .collect::<Vec<_>>())
    }
}

#[derive(InputObject)]
pub struct PersonInput {
    first_name: String,
    last_name: String,
}

fn random_from_ids(ids: Vec<String>) -> String {
    loop {
        let id = random::<u16>().to_string();
        let contains = ids.iter().any(|inner| inner == &id);
        if !contains {
            break id;
        }
    }
}

impl PersonInput {
    fn to_person_raw_excluding_ids(self, ids: Vec<String>) -> database::Person {
        let id = random_from_ids(ids);
        database::Person {
            id,
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
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

pub struct Mutation;

#[Object]
impl Mutation {
    async fn add_person<'ctx>(
        &self,
        context: &Context<'ctx>,
        person: PersonInput,
    ) -> Result<String> {
        let graph = context.data::<GraphDB>()?;
        let person_raw = person.to_person_raw_excluding_ids(graph.person_ids());
        graph.insert_person(person_raw.clone());
        Ok(person_raw.id)
    }
}

// stream = person + enter or exit

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum)]
enum Status {
    Open,
    Closed,
}

impl Status {
    fn switch(self) -> Self {
        match self {
            Status::Closed => Status::Open,
            Status::Open => Status::Closed,
        }
    }
}

struct Subscription;

#[Subscription]
impl Subscription {
    async fn integers(&self) -> impl Stream<Item = Status> {
        let mut value = Status::Open;
        let interval = tokio::time::interval(Duration::from_secs(10));
        tokio_stream::wrappers::IntervalStream::new(interval).map(move |_| {
            value = value.switch();
            value
        })
    }
}
