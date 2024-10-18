use o2o::o2o;
use uuid::Uuid;
use crate::repository::entities::notification_entity::NotificationEntity;

#[derive(o2o)]
#[from_owned(NotificationEntity)]
pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

pub struct CreateNotification {
    pub title: String,
    pub body: String,
}
