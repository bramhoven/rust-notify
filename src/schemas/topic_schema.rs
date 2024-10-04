use serde::Serialize;

#[derive(Debug, Serialize)]
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
