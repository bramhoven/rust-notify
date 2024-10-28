use deadpool_diesel::postgres::Pool;
use uuid::Uuid;
use log::error as error_log;

use crate::{errors::ServiceError, models::topic::{CreateTopic, Topic}, repository::{entities::topic_entity::TopicEntity, stores::topic_store::TopicStore}};

#[derive(Clone)]
pub struct TopicService {
    pooled_connection: Pool,
}

impl TopicService {
    pub fn new(pooled_connection: Pool) -> Self {
        Self {
            pooled_connection
        }
    }

    pub async fn get_topics(&self) -> Result<Vec<Topic>, ServiceError> {
        let conn = self.pooled_connection.get().await.unwrap();
        let store = TopicStore::new();

        let mut error: Option<diesel::result::Error> = None;
        let topics: Vec<TopicEntity> = match conn.interact(move |conn| {
            store.get_topics(conn)
        }).await.unwrap() {
            Ok(topics) => match topics {
                Some(topics) => topics,
                None => Vec::<TopicEntity>::new(),
            },
            Err(err) => {
                error_log!("Failed to get topics: {}", err);
                error = Some(err);
                Vec::<TopicEntity>::new()
            }
        };

        if error.is_some() {
            return Err(ServiceError::Error);
        }

        let mut mapped_topics: Vec<Topic> = vec![];
        for topic in topics.into_iter() {
            let topic = topic.into();
            mapped_topics.push(topic);
        }

        Ok(mapped_topics)
    }

    pub async fn get_topic(&self, topic_id: Uuid) -> Result<Topic, ServiceError> {
        let conn = self.pooled_connection.get().await.unwrap();
        let store = TopicStore::new();

        let mut error: Option<diesel::result::Error> = None;
        let topic: Option<TopicEntity> = match conn.interact(move |conn| {
            store.get_topic(conn, topic_id)
        }).await.unwrap() {
            Ok(topic) => topic,
            Err(err) => {
                error_log!("Failed to get topic {}: {}", topic_id, err);
                error = Some(err);
                None
            }
        };

        if topic.is_some() {
            let topic = topic.unwrap().into();
            return Ok(topic);
        }
        else if error.is_none() {
            return Err(ServiceError::NotFound);
        }

        Err(ServiceError::Error)
    }

    pub async fn add_topic(&self, create_topic: CreateTopic) -> Result<Topic, ServiceError> {
        let conn = self.pooled_connection.get().await.unwrap();
        let store = TopicStore::new();

        let mut error: Option<diesel::result::Error> = None;

        let topic: Option<TopicEntity> = match conn.interact(move |conn| {
            store.add_topic(conn, create_topic)
        }).await.unwrap() {
            Ok(topic) => Some(topic),
            Err(err) => {
                error_log!("Failed to add topic: {}", err);
                error = Some(err);
                None
            }
        };

        if topic.is_some() {
            let notification = topic.unwrap().into();
            return Ok(notification)
        }

        return match error.unwrap() {
            diesel::result::Error::DatabaseError(db_error, _) => {
                match db_error {
                    diesel::result::DatabaseErrorKind::UniqueViolation => Err(ServiceError::NotUnique),
                    _ => Err(ServiceError::Error),
                }
            },
            _ => Err(ServiceError::Error),
        }
    }

    pub async fn update_topic(&self, topic_id: Uuid, update_topic: CreateTopic) -> Result<Topic, ServiceError> {
        let conn = self.pooled_connection.get().await.unwrap();
        let store = TopicStore::new();

        let mut error: Option<diesel::result::Error> = None;

        let topic: Option<TopicEntity> = match conn.interact(move |conn| {
            store.update_topic(conn, topic_id, update_topic)
        }).await.unwrap() {
            Ok(topic) => Some(topic),
            Err(err) => {
                error = Some(err);
                None
            }
        };

        if topic.is_some() {
            let topic = topic.unwrap().into();
            return Ok(topic);
        }

        return match error.unwrap() {
            diesel::result::Error::DatabaseError(db_error, _) => {
                match db_error {
                    diesel::result::DatabaseErrorKind::UniqueViolation => Err(ServiceError::NotUnique),
                    _ => Err(ServiceError::Error),
                }
            },
            _ => Err(ServiceError::Error),
        }
    }
}

