use axum::{Json, extract::{State, Json as ExtractJson, Path}, http::StatusCode};
use uuid::Uuid;

use crate::{app::AppState, errors::ServiceError, schemas::{error_schema::ErrorSchema, topic_schema::{CreateTopicSchema, TopicSchema}}};


pub async fn get_topics(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<TopicSchema>>), (StatusCode, Json<ErrorSchema>)> {
    let mut error: Option<ServiceError> = None;
    let topics = match state.topic_service.get_topics().await {
        Ok(topics) => Some(topics),
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if topics.is_some() {
        let mut mapped_topics: Vec<TopicSchema> = vec![];
        for topic in topics.unwrap().into_iter() {
            let topic = TopicSchema::from(topic);
            mapped_topics.push(topic);
        }

        return Ok((StatusCode::OK, Json(mapped_topics)));
    }
    else if topics.is_none() && error.is_none() {
        return Ok((StatusCode::OK, Json(Vec::<TopicSchema>::new())));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to get topics") })))
}

pub async fn get_topic(State(state): State<AppState>, Path(topic_id): Path<Uuid>) -> Result<(StatusCode, Json<TopicSchema>), (StatusCode, Json<ErrorSchema>)>  {
    let mut error: Option<ServiceError> = None;
    let topic = match state.topic_service.get_topic(topic_id).await {
        Ok(topic) => Some(topic),
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if topic.is_some() {
        let topic = TopicSchema::from(topic.unwrap());

        return Ok((StatusCode::OK, Json(topic)));
    } 
    else if topic.is_none() && error.is_none() {
        return Err((StatusCode::NOT_FOUND, Json(ErrorSchema { error: String::from("Topic not found") })));
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to get topic") })))
}

pub async fn add_topic(State(state): State<AppState>, ExtractJson(create_topic_schema): ExtractJson<CreateTopicSchema>) -> Result<(StatusCode, Json<TopicSchema>), (StatusCode, Json<ErrorSchema>)> {
    let create_topic = create_topic_schema.clone().into();
    let mut error: Option<ServiceError> = None;
    let topic = match state.topic_service.add_topic(create_topic).await {
        Ok(topic) => Some(topic),
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if topic.is_some() {
        let topic = TopicSchema::from(topic.unwrap());
        return Ok((StatusCode::OK, Json(topic)));
    }

    return match error.unwrap() {
        ServiceError::NotUnique => Err((StatusCode::BAD_REQUEST, Json(ErrorSchema { error: format!("Topic with name '{}' already exists", create_topic_schema.name) }))),
        _ => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to add topic") }))),
    }
}

pub async fn update_topic(State(state): State<AppState>, Path(topic_id): Path<Uuid>, ExtractJson(update_topic_schema): ExtractJson<CreateTopicSchema>) -> Result<(StatusCode, Json<TopicSchema>), (StatusCode, Json<ErrorSchema>)> {
    let update_topic = update_topic_schema.clone().into();
    let mut error: Option<ServiceError> = None;
    let topic = match state.topic_service.update_topic(topic_id, update_topic).await {
        Ok(topic) => Some(topic),
        Err(err) => {
            error = Some(err);
            None
        }
    };

    if topic.is_some() {
        let topic = TopicSchema::from(topic.unwrap());
        return Ok((StatusCode::OK, Json(topic)));
    }

    return match error.unwrap() {
        ServiceError::NotUnique => Err((StatusCode::BAD_REQUEST, Json(ErrorSchema { error: format!("Topic with name '{}' already exists", update_topic_schema.name) }))),
        _ => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorSchema { error: String::from("Failed to update topic") }))),
    }
}
