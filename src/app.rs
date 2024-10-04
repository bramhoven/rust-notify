use axum::{
    routing::get,
    http::StatusCode,
    Router,
};
use tower_http::trace::TraceLayer;

use crate::routes::topic_routes;

pub async fn create_app() -> Router {
    let app = Router::new()
        .route("/", get(root))
        .route("/topics", get(topic_routes::get_topics))
        .layer(TraceLayer::new_for_http());

    app
}

async fn root() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, World!")
}

