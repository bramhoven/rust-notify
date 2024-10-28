use axum::{Json, extract::{State, Json as ExtractJson, Path}, http::StatusCode};
use uuid::Uuid;

use crate::{app::AppState, errors::ServiceError, models::notification::{CreateNotification, Notification}, schemas::{error_schema::ErrorSchema, notification_schema::{CreateNotificationSchema, NotificationSchema}}};


pub async fn get_notifications(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<NotificationSchema>>), (StatusCode, Json<ErrorSchema>)> {
    let mut error: Option<ServiceError> = None;
    let notifications = match state.notification_service.get_notifications().await {
        Ok(notifications) => Some(notifications),
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if notifications.is_some() {
        let mut mapped_notifications: Vec<NotificationSchema> = vec![];
        for notification in notifications.unwrap().into_iter() {
            let notification = NotificationSchema::from(notification);
            mapped_notifications.push(notification);
        }

        return Ok((StatusCode::OK, Json(mapped_notifications)));
    }
    else if notifications.is_none() && error.is_none() {
        return Ok((StatusCode::OK, Json(Vec::<NotificationSchema>::new())));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to get notifications") })))
}

pub async fn get_notification(State(state): State<AppState>, Path(notification_id): Path<Uuid>) -> Result<(StatusCode, Json<NotificationSchema>), (StatusCode, Json<ErrorSchema>)>  {
    let mut error: Option<ServiceError> = None;
    let notification = match state.notification_service.get_notification(notification_id).await {
        Ok(notification) => Some(notification),
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if notification.is_some() {
        let notification = NotificationSchema::from(notification.unwrap());

        return Ok((StatusCode::OK, Json(notification)));
    } 
    else if notification.is_none() && error.is_none() {
        return Err((StatusCode::NOT_FOUND, Json(ErrorSchema { error: format!("Notification '{}' not found", notification_id) })));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: format!("Failed to get notification '{}'", notification_id) })))
}

pub async fn add_notification(State(state): State<AppState>, ExtractJson(create_notification_schema): ExtractJson<CreateNotificationSchema>) -> Result<(StatusCode, Json<NotificationSchema>), (StatusCode, Json<ErrorSchema>)> {
    let create_notification = create_notification_schema.clone().into();
    let notification = match state.notification_service.add_notification(create_notification).await {
        Ok(notification) => Some(notification),
        Err(_) => None
    };

    if notification.is_some() {
        let notification = Notification::from(notification.unwrap());
        let notification = NotificationSchema::from(notification);

        return Ok((StatusCode::OK, Json(notification)));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to add notification") })))
}

pub async fn update_notification(State(state): State<AppState>, Path(notification_id): Path<Uuid>, ExtractJson(update_notification_schema): ExtractJson<CreateNotificationSchema>) -> Result<(StatusCode, Json<NotificationSchema>), (StatusCode, Json<ErrorSchema>)> {
    let update_notification: CreateNotification = update_notification_schema.clone().into();
    let notification = match state.notification_service.update_notification(notification_id, update_notification).await {
        Ok(notification) => Some(notification),
        Err(_) => None
    };

    if notification.is_some() {
        let notification = Notification::from(notification.unwrap());
        let notification = NotificationSchema::from(notification);

        return Ok((StatusCode::OK, Json(notification)));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: format!("Failed to update notification '{}'", notification_id) })))
}
