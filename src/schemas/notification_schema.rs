use serde::{Deserialize, Serialize};
use o2o::o2o;
use uuid::Uuid;
use crate::models::notification::{Notification, CreateNotification};

#[derive(Debug, Serialize, o2o)]
#[from_owned(Notification)]
pub struct NotificationSchema {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Clone, o2o)]
#[owned_into(CreateNotification)]
pub struct CreateNotificationSchema {
    pub title: String,
    pub body: String,
}
