use serde::{Deserialize, Serialize};
use o2o::o2o;
use uuid::Uuid;
use crate::models::topic::{Topic, CreateTopic};

#[derive(Debug, Serialize, o2o)]
#[from_owned(Topic)]
pub struct TopicSchema {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, o2o)]
#[owned_into(CreateTopic)]
pub struct CreateTopicSchema {
    pub name: String,
}
