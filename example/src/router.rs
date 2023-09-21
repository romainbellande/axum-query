use crate::graphql;
use crate::State;
use axum::Extension;
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

pub fn router(state: State) -> Router {
    Router::new()
        .nest("/graphql", graphql::router(state.clone()))
        .layer(Extension(state))
        .layer(CorsLayer::permissive())
}
