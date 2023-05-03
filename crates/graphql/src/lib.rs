use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn todo(&self) -> &'static str {
        todo!()
    }
}
