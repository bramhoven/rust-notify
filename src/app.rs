use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    Router,
};
use tower_http::trace::TraceLayer;
use deadpool_diesel::postgres::Pool;

use crate::{routes::{notification_routes, topic_routes}, services::{notification_service::NotificationService, topic_service::TopicService}};

#[derive(Clone)]
pub struct AppState {
    pub notification_service: NotificationService,
    pub topic_service: TopicService,
}

pub async fn create_app(pooled_connection: Pool) -> Router {
    let state = AppState {
        notification_service: NotificationService::new(pooled_connection.clone()),
        topic_service: TopicService::new(pooled_connection.clone()),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/topics", get(topic_routes::get_topics))
        .route("/topics", post(topic_routes::add_topic))
        .route("/topics/:topic_id", get(topic_routes::get_topic))
        .route("/topics/:topic_id", put(topic_routes::update_topic))
        .route("/topics/:topic_id", delete(topic_routes::delete_topic))
        .route("/notifications", get(notification_routes::get_notifications))
        .route("/notifications", post(notification_routes::add_notification))
        .route("/notifications/:notification_id", get(notification_routes::get_notification))
        .route("/notifications/:notification_id", put(notification_routes::update_notification))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    app
}

async fn root() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, World!")
}

