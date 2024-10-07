use diesel::prelude::*;

use crate::{repository::entities::topic_entity::{TopicEntity, NewTopicEntity}, models::topic::CreateTopic};

pub struct TopicStore {}

impl TopicStore {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_topics(&self, connection: &mut PgConnection) -> Result<Option<Vec<TopicEntity>>, diesel::result::Error> {
        use crate::repository::schema::topics::dsl::*;

        topics
            .select(TopicEntity::as_select())
            .load(connection)
            .optional()
    }

    pub fn add_topic(&self, connection: &mut PgConnection, create_topic: CreateTopic) -> Result<TopicEntity, diesel::result::Error> {
        use crate::repository::schema::topics;

        let new_topic = NewTopicEntity {
            name: create_topic.name.as_str()
        };

        diesel::insert_into(topics::table)
            .values(&new_topic)
            .returning(TopicEntity::as_returning())
            .get_result(connection)
    }
}
