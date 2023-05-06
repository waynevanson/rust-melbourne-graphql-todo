use std::sync::RwLock;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use database::*;
use graphql::{AppSchema, Mutation, Pool, Query};

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    let graphiql = GraphiQLSource::build()
        .endpoint("/")
        .subscription_endpoint("/ws")
        .finish();

    response::Html(graphiql)
}

#[tokio::main]
async fn main() {
    let db = Database::new(
        vec![User {
            id: 323,
            first_name: "Jason".to_string(),
            last_name: "Statham".to_string(),
        }],
        vec![],
        vec![],
    );

    let db = Pool(RwLock::new(db));

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db)
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
