// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    channels (id) {
        id -> Uuid,
        #[max_length = 32]
        name -> Varchar,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        user_id -> Uuid,
        channel_id -> Uuid,
        #[max_length = 1024]
        content -> Varchar,
    }
}

diesel::table! {
    secrets (user_id) {
        user_id -> Uuid,
        salt -> Bytea,
    }
}

diesel::table! {
    sessions (key) {
        user_id -> Uuid,
        key -> Bytea,
        nonce -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    user_channel (user_id, channel_id) {
        user_id -> Uuid,
        channel_id -> Uuid,
        role -> UserRole,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 64]
        username -> Varchar,
        is_admin -> Bool,
    }
}

diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(secrets -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(user_channel -> channels (channel_id));
diesel::joinable!(user_channel -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    messages,
    secrets,
    sessions,
    user_channel,
    users,
);
