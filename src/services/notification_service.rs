use deadpool_diesel::postgres::Pool;
use uuid::Uuid;
use log::error as error_log;

use crate::models::notification::{CreateNotification, Notification};
use crate::repository::entities::notification_entity::NotificationEntity;
use crate::repository::stores::notification_store::NotificationStore;
use crate::errors::ServiceError;

#[derive(Clone)]
pub struct NotificationService {
    pooled_connection: Pool,
}

impl NotificationService {
    pub fn new(pooled_connection: Pool) -> Self {
        Self {
            pooled_connection
        }
    }

    pub async fn get_notifications(&self) -> Result<Vec<Notification>, ServiceError> {
        let conn = self.pooled_connection.get().await.unwrap();
        let store = NotificationStore::new();

       let mut error: Option<diesel::result::Error> = None;
        let notifications: Vec<NotificationEntity> = match conn.interact(move |conn| {
            store.get_notifications(conn)
        }).await.unwrap() {
            Ok(notifications) => match notifications {
                Some(notifications) => notifications,
                None => Vec::<NotificationEntity>::new(),
            },
            Err(err) => {
                error_log!("Failed to get notifications: {}", err);
                error = Some(err);
                Vec::<NotificationEntity>::new()
            },
        };

        let mut mapped_notifications: Vec<Notification> = vec![];
        for notification in notifications.into_iter() {
            let notification = notification.into();
            mapped_notifications.push(notification);
        }

        if error.is_some() {
            return Err(ServiceError::Error);
        }

        Ok(mapped_notifications)
    }

    pub async fn get_notification(&self, notification_id: Uuid) -> Result<Notification, ServiceError> {
        let conn = self.pooled_connection.get().await.unwrap();
        let store = NotificationStore::new();

        let mut error: Option<diesel::result::Error> = None;
        let notification: Option<NotificationEntity> = match conn.interact(move |conn| {
            store.get_notification(conn, notification_id)
        }).await.unwrap() {
            Ok(notification) => notification,
            Err(err) => {
                error_log!("Failed to get notification {}: {}", notification_id, err);
                error = Some(err);
                None
            }
        };

        if notification.is_some() {
            let notification = notification.unwrap().into();
            return Ok(notification)
        }
        else if notification.is_none() && error.is_none() {
            return Err(ServiceError::NotFound);
        }

        Err(ServiceError::Error)
    }

    pub async fn add_notification(&self, create_notification: CreateNotification) -> Result<Notification, ServiceError> {
        let conn = self.pooled_connection.get().await.unwrap();
        let store = NotificationStore::new();

        let notification: Option<NotificationEntity> = match conn.interact(move |conn| {
            store.add_notification(conn, create_notification)
        }).await.unwrap() {
            Ok(notification) => Some(notification),
            Err(err) => {
                error_log!("Failed to add notification: {}", err);
                None
            }
        };

        if notification.is_some() {
            let notification = notification.unwrap().into();
            return Ok(notification)
        }

        Err(ServiceError::Error)
    }

    pub async fn update_notification(&self, notification_id: Uuid, update_notification: CreateNotification) -> Result<Notification, ServiceError> {
        let conn = self.pooled_connection.get().await.unwrap();
        let store = NotificationStore::new();

        let notification: Option<NotificationEntity> = match conn.interact(move |conn| {
            store.update_notification(conn, notification_id, update_notification)
        }).await.unwrap() {
            Ok(notification) => Some(notification),
            Err(err) => {
                error_log!("Failed to update notification {}: {}", notification_id, err);
                None
            }
        };

        if notification.is_some() {
            let notification = notification.unwrap().into();
            return Ok(notification)
        }

        Err(ServiceError::Error)
    }
}
