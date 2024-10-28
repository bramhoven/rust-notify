use uuid::Uuid;

pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

pub struct CreateNotification {
    pub title: String,
    pub body: String,
}
