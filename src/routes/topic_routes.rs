use axum::{Json, extract::State};

use crate::{schemas::topic_schema::TopicSchema, app::AppState, repository::{stores::topic_store::TopicStore, entities::topic_entity::TopicEntity}, models::topic::Topic};


pub async fn get_topics(State(state): State<AppState>) -> Json<Vec<TopicSchema>> {
    // TODO: Not have direct DB call in route controller
    let conn = state.pooled_connection.get().await.unwrap();
    let store = TopicStore::new();

    let topics: Vec<TopicEntity> = match conn.interact(move |conn| {
        store.get_topics(conn)
    }).await.unwrap() {
        Ok(topics) => match topics {
            Some(topics) => topics,
            None => Vec::<TopicEntity>::new(),
        },
        Err(_) => Vec::<TopicEntity>::new(),
    };

    let mut mapped_topics: Vec<TopicSchema> = vec![];

    for topic in topics.into_iter() {
        let model = Topic::from(topic);
        let schema = TopicSchema::from(model);
        mapped_topics.push(schema);
    }

    Json(mapped_topics)
}

pub async fn add_topic() -> Json<TopicSchema> {
    Json(TopicSchema::new("".to_string()))
}
