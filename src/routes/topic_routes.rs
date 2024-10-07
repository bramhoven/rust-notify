use axum::{Json, extract::{State, Json as ExtractJson}, http::StatusCode};

use crate::{schemas::{topic_schema::{TopicSchema, CreateTopicSchema}, error_schema::ErrorSchema}, app::AppState, repository::{stores::topic_store::TopicStore, entities::topic_entity::TopicEntity}, models::topic::{Topic, CreateTopic}};


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
        let topic = Topic::from(topic);
        let topic = TopicSchema::from(topic);
        mapped_topics.push(topic);
    }

    Json(mapped_topics)
}

pub async fn add_topic(State(state): State<AppState>, ExtractJson(create_topic): ExtractJson<CreateTopicSchema>) -> Result<(StatusCode, Json<TopicSchema>), (StatusCode, Json<ErrorSchema>)> {
    // TODO: Not have direct DB call in route controller
    let conn = state.pooled_connection.get().await.unwrap();
    let store = TopicStore::new();

    let create_topic: CreateTopic = create_topic.into();
    let topic: Option<TopicEntity> = match conn.interact(move |conn| {
        store.add_topic(conn, create_topic)
    }).await.unwrap() {
        Ok(topic) => Some(topic),
        Err(_) => None
    };

    if topic.is_some() {
        let topic = Topic::from(topic.unwrap());
        let topic = TopicSchema::from(topic);
        Ok((StatusCode::OK, Json(topic)))
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to add topic") })))
    }
}
