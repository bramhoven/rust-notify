use log::info;
use tracing::Level;
use dotenvy::dotenv;
use std::env;
use clap::Parser;

mod app;
mod cli;
mod routes;
mod schemas;
mod repository;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = cli::arguments::CommandLineArguments::parse();

    // Configure tracing logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // Create bind string: 0.0.0.0:3000
    let mut bind_str: String = args.host;
    bind_str.push_str(":");
    bind_str.push_str(args.port.to_string().as_str());

    // Create database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _conn = repository::database::establish_connection(&database_url);

    // Create server
    let server = app::create_app().await;

    info!(target: "rust-notify", "starting server on: {}", bind_str);

    // Start tokio listener
    let listener = tokio::net::TcpListener::bind(bind_str).await.unwrap();
    axum::serve(listener, server).await.unwrap();
}

