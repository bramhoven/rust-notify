use axum::Json;

use crate::schemas::topic_schema::TopicSchema;

pub async fn get_topics() -> Json<Vec<TopicSchema>> {
    let mut topics: Vec<TopicSchema> = vec![];

    let tmp_topic = TopicSchema::new("test-topic".to_string());
    topics.push(tmp_topic);

    Json(topics)
}
