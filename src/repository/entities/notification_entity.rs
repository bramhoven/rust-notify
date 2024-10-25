use diesel::prelude::*;
use uuid::Uuid;
use o2o::o2o;

use crate::models::notification::Notification;

#[derive(Queryable, Selectable, o2o)]
#[diesel(table_name = crate::repository::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[owned_into(Notification)]
pub struct NotificationEntity {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::repository::schema::notifications)]
pub struct NewNotificationEntity<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
