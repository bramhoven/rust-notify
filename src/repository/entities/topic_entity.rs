use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::repository::schema::topics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TopicEntity {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::repository::schema::topics)]
pub struct NewTopicEntity<'a> {
    pub name: &'a str,
}
