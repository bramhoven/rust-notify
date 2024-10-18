use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::repository::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
