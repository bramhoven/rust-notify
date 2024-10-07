use serde::Serialize;
use o2o::o2o;
use crate::models::topic::Topic;

#[derive(Debug, Serialize, o2o)]
#[from_owned(Topic)]
pub struct TopicSchema {
    name: String,
}

impl TopicSchema {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}
