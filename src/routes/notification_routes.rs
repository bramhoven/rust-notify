use axum::{Json, extract::{State, Json as ExtractJson, Path}, http::StatusCode};
use uuid::Uuid;

use crate::{schemas::{notification_schema::{NotificationSchema, CreateNotificationSchema}, error_schema::ErrorSchema}, app::AppState, repository::{stores::notification_store::NotificationStore, entities::notification_entity::NotificationEntity}, models::notification::{Notification, CreateNotification}};


pub async fn get_notifications(State(state): State<AppState>) -> Json<Vec<NotificationSchema>> {
    // TODO: Not have direct DB call in route controller
    let conn = state.pooled_connection.get().await.unwrap();
    let store = NotificationStore::new();

    let notifications: Vec<NotificationEntity> = match conn.interact(move |conn| {
        store.get_notifications(conn)
    }).await.unwrap() {
        Ok(notifications) => match notifications {
            Some(notifications) => notifications,
            None => Vec::<NotificationEntity>::new(),
        },
        Err(_) => Vec::<NotificationEntity>::new(),
    };

    let mut mapped_notifications: Vec<NotificationSchema> = vec![];

    for notification in notifications.into_iter() {
        let notification = Notification::from(notification);
        let notification = NotificationSchema::from(notification);
        mapped_notifications.push(notification);
    }

    Json(mapped_notifications)
}

pub async fn get_notification(State(state): State<AppState>, Path(notification_id): Path<Uuid>) -> Result<(StatusCode, Json<NotificationSchema>), (StatusCode, Json<ErrorSchema>)>  {
    // TODO: Not have direct DB call in route controller
    let conn = state.pooled_connection.get().await.unwrap();
    let store = NotificationStore::new();

    let mut error: Option<diesel::result::Error> = None;
    let notification: Option<NotificationEntity> = match conn.interact(move |conn| {
        store.get_notification(conn, notification_id)
    }).await.unwrap() {
        Ok(notification) => notification,
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if notification.is_some() {
        let notification = Notification::from(notification.unwrap());
        let notification = NotificationSchema::from(notification);

        return Ok((StatusCode::OK, Json(notification)));
    } 
    else if notification.is_none() && error.is_none() {
        return Err((StatusCode::NOT_FOUND, Json(ErrorSchema { error: String::from("Notification not found") })));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to get notification") })))
}

pub async fn add_notification(State(state): State<AppState>, ExtractJson(create_notification_schema): ExtractJson<CreateNotificationSchema>) -> Result<(StatusCode, Json<NotificationSchema>), (StatusCode, Json<ErrorSchema>)> {
    // TODO: Not have direct DB call in route controller
    let conn = state.pooled_connection.get().await.unwrap();
    let store = NotificationStore::new();

    let mut error: Option<diesel::result::Error> = None;

    let create_notification: CreateNotification = create_notification_schema.clone().into();
    let notification: Option<NotificationEntity> = match conn.interact(move |conn| {
        store.add_notification(conn, create_notification)
    }).await.unwrap() {
        Ok(notification) => Some(notification),
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if notification.is_some() {
        let notification = Notification::from(notification.unwrap());
        let notification = NotificationSchema::from(notification);

        return Ok((StatusCode::OK, Json(notification)));
    }
    else if error.is_some() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to add notification") })));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to add notification") })))
}

pub async fn update_notification(State(state): State<AppState>, Path(notification_id): Path<Uuid>, ExtractJson(update_notification_schema): ExtractJson<CreateNotificationSchema>) -> Result<(StatusCode, Json<NotificationSchema>), (StatusCode, Json<ErrorSchema>)> {
    // TODO: Not have direct DB call in route controller
    let conn = state.pooled_connection.get().await.unwrap();
    let store = NotificationStore::new();

    let mut error: Option<diesel::result::Error> = None;

    let update_notification: CreateNotification = update_notification_schema.clone().into();
    let notification: Option<NotificationEntity> = match conn.interact(move |conn| {
        store.update_notification(conn, notification_id, update_notification)
    }).await.unwrap() {
        Ok(notification) => Some(notification),
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if notification.is_some() {
        let notification = Notification::from(notification.unwrap());
        let notification = NotificationSchema::from(notification);

        return Ok((StatusCode::OK, Json(notification)));
    }
    else if error.is_some() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to update notification") })));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to update notification") })))
}
