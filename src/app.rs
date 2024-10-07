use axum::{
    routing::{get, post},
    http::StatusCode,
    Router,
};
use tower_http::trace::TraceLayer;
use deadpool_diesel::postgres::Pool;

use crate::routes::topic_routes;

#[derive(Clone)]
pub struct AppState {
    pub pooled_connection: Pool,
}

pub async fn create_app(pooled_connection: Pool) -> Router {
    let state = AppState {
        pooled_connection
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/topics", get(topic_routes::get_topics))
        .route("/topics", post(topic_routes::add_topic))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    app
}

async fn root() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, World!")
}

