// @generated automatically by Diesel CLI.

diesel::table! {
    notifications (id) {
        id -> Uuid,
        created_at -> Timestamp,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 500]
        body -> Varchar,
    }
}

diesel::table! {
    topics (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    notifications,
    topics,
);
