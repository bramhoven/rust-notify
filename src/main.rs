use axum::{
    routing::get,
    http::StatusCode,
    Router,
};
use log::info;
use tower_http::trace::TraceLayer;
use tracing::Level;

use clap::Parser;

mod cli;
mod routes;
mod schemas;

use routes::topic_routes;

#[tokio::main]
async fn main() {
    let args = cli::arguments::CommandLineArguments::parse();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // Create bind string: 0.0.0.0:3000
    let mut bind_str: String = args.host;
    bind_str.push_str(":");
    bind_str.push_str(args.port.to_string().as_str());


    let app = create_app().await;

    info!(target: "rust-notify", "starting server on: {}", bind_str);

    let listener = tokio::net::TcpListener::bind(bind_str).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_app() -> Router {
    let app = Router::new()
        .route("/", get(root))
        .route("/topics", get(topic_routes::get_topics))
        .layer(TraceLayer::new_for_http());

    app
}

async fn root() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, World!")
}

