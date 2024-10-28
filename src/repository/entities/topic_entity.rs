use diesel::prelude::*;
use o2o::o2o;
use uuid::Uuid;

use crate::models::topic::Topic;

#[derive(Queryable, Selectable, o2o)]
#[diesel(table_name = crate::repository::schema::topics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[owned_into(Topic)]
pub struct TopicEntity {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::repository::schema::topics)]
pub struct NewTopicEntity<'a> {
    pub name: &'a str,
}
