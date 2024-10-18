use diesel::prelude::*;
use uuid::Uuid;

use crate::{repository::entities::notification_entity::{NotificationEntity, NewNotificationEntity}, models::notification::CreateNotification};

pub struct NotificationStore {}

impl NotificationStore {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_notifications(&self, connection: &mut PgConnection) -> Result<Option<Vec<NotificationEntity>>, diesel::result::Error> {
        use crate::repository::schema::notifications::dsl::*;

        notifications
            .select(NotificationEntity::as_select())
            .load(connection)
            .optional()
    }

    pub fn get_notification(&self, connection: &mut PgConnection, notification_id: Uuid) -> Result<Option<NotificationEntity>, diesel::result::Error> {
        use crate::repository::schema::notifications::dsl::*;

        notifications
            .select(NotificationEntity::as_select())
            .find(notification_id)
            .first(connection)
            .optional()
    }

    pub fn add_notification(&self, connection: &mut PgConnection, create_notification: CreateNotification) -> Result<NotificationEntity, diesel::result::Error> {
        use crate::repository::schema::notifications;

        let new_notification = NewNotificationEntity {
            title: create_notification.title.as_str(),
            body: create_notification.body.as_str()
        };

        diesel::insert_into(notifications::table)
            .values(&new_notification)
            .returning(NotificationEntity::as_returning())
            .get_result(connection)
    }

    pub fn update_notification(&self, connection: &mut PgConnection, notification_id: Uuid, update_notification: CreateNotification) -> Result<NotificationEntity, diesel::result::Error> {
        use crate::repository::schema::notifications::dsl::{notifications, title, body};

        diesel::update(notifications.find(notification_id))
            .set((
                title.eq(update_notification.title),
                body.eq(update_notification.body)
            ))
            .returning(NotificationEntity::as_returning())
            .get_result(connection)
    }
}
