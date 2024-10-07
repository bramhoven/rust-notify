use serde::{Deserialize, Serialize};
use o2o::o2o;
use uuid::Uuid;
use crate::models::topic::{Topic, CreateTopic};
use crate::utils::serializable_uuid;

#[derive(Debug, Serialize, o2o)]
#[from_owned(Topic)]
pub struct TopicSchema {
    #[serde(with = "serializable_uuid")]
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, o2o)]
#[owned_into(CreateTopic)]
pub struct CreateTopicSchema {
    pub name: String,
}
