use axum::{
    routing::{get},
    http::StatusCode,
    Router,
};
use clap::Parser;
mod cli;

#[tokio::main]
async fn main() {
    let args = cli::arguments::CommandLineArguments::parse();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

    // Create bind string: 0.0.0.0:3000
    let mut bind_str: String = args.host;
    bind_str.push_str(":");
    bind_str.push_str(args.port.to_string().as_str());

    println!("Server started on: {}", bind_str);
    let listener = tokio::net::TcpListener::bind(bind_str).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, World!")
}
