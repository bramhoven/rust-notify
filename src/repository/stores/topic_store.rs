use diesel::prelude::*;

use crate::repository::entities::topic_entity::TopicEntity;

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
}
