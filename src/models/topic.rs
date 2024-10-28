use uuid::Uuid;

pub struct Topic {
    pub id: Uuid,
    pub name: String,
}

pub struct CreateTopic {
    pub name: String,
}
