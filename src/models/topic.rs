use o2o::o2o;
use uuid::Uuid;
use crate::repository::entities::topic_entity::TopicEntity;

#[derive(o2o)]
#[from_owned(TopicEntity)]
pub struct Topic {
    pub id: Uuid,
    pub name: String,
}

pub struct CreateTopic {
    pub name: String,
}
