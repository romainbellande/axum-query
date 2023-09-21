mod schema;
pub mod types;


use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::header::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use schema::{get_schema, AppSchema};
use crate::State;

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    schema.execute(req).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub fn router(state: State) -> Router {
    let schema = get_schema(state);

    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
}
