use axum::{
    routing::{get},
    http::StatusCode,
    Router, Json
};
use serde::{Serialize};
use log::{info};
use tower_http::trace::TraceLayer;
use tracing::Level;

use clap::Parser;
mod cli;

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
        .route("/topics", get(get_topics))
        .layer(TraceLayer::new_for_http());

    app
}

async fn root() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, World!")
}

async fn get_topics() -> Json<Vec<Topic>> {
    let mut topics: Vec<Topic> = vec![];

    let tmp_topic = Topic::new("test-topic".to_string());
    topics.push(tmp_topic);

    Json(topics)
}

#[derive(Debug, Serialize)]
struct Topic {
    name: String,
}

impl Topic {
    fn new(name: String) -> Self {
        Self {
            name
        }
    }
}


