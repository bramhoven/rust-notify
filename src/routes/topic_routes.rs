use axum::{Json, extract::{State, Json as ExtractJson, Path}, http::StatusCode};
use uuid::Uuid;

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

pub async fn get_topic(State(state): State<AppState>, Path(topic_id): Path<Uuid>) -> Result<(StatusCode, Json<TopicSchema>), (StatusCode, Json<ErrorSchema>)>  {
    // TODO: Not have direct DB call in route controller
    let conn = state.pooled_connection.get().await.unwrap();
    let store = TopicStore::new();

    let mut error: Option<diesel::result::Error> = None;
    let topic: Option<TopicEntity> = match conn.interact(move |conn| {
        store.get_topic(conn, topic_id)
    }).await.unwrap() {
        Ok(topic) => topic,
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if topic.is_some() {
        let topic = Topic::from(topic.unwrap());
        let topic = TopicSchema::from(topic);

        return Ok((StatusCode::OK, Json(topic)));
    } 
    else if topic.is_none() && error.is_none() {
        return Err((StatusCode::NOT_FOUND, Json(ErrorSchema { error: String::from("Topic not found") })));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to get topic") })))
}

pub async fn add_topic(State(state): State<AppState>, ExtractJson(create_topic_schema): ExtractJson<CreateTopicSchema>) -> Result<(StatusCode, Json<TopicSchema>), (StatusCode, Json<ErrorSchema>)> {
    // TODO: Not have direct DB call in route controller
    let conn = state.pooled_connection.get().await.unwrap();
    let store = TopicStore::new();

    let mut error: Option<diesel::result::Error> = None;

    let create_topic: CreateTopic = create_topic_schema.clone().into();
    let topic: Option<TopicEntity> = match conn.interact(move |conn| {
        store.add_topic(conn, create_topic)
    }).await.unwrap() {
        Ok(topic) => Some(topic),
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if topic.is_some() {
        let topic = Topic::from(topic.unwrap());
        let topic = TopicSchema::from(topic);

        return Ok((StatusCode::OK, Json(topic)));
    }
    else if topic.is_none() && error.is_none() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to add topic") })));
    }
    else if error.is_some() {
        return match error.unwrap() {
            diesel::result::Error::DatabaseError(db_error, _) => {
                match db_error {
                    diesel::result::DatabaseErrorKind::UniqueViolation => Err((StatusCode::BAD_REQUEST, Json(ErrorSchema { error: format!("Topic with name '{}' already exists", create_topic_schema.name) }))),
                    _ => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to add topic") }))),
                }
            },
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to add topic") }))),
        }
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to add topic") })))
}
