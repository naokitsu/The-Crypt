// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "member_role"))]
    pub struct MemberRole;
}

diesel::table! {
    bans (user_id, channel_id) {
        user_id -> Uuid,
        channel_id -> Uuid,
    }
}

diesel::table! {
    channels (id) {
        id -> Uuid,
        #[max_length = 32]
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MemberRole;

    members (user_id, channel_id) {
        user_id -> Uuid,
        channel_id -> Uuid,
        role -> MemberRole,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        user_id -> Uuid,
        channel_id -> Uuid,
        content -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 32]
        name -> Varchar,
    }
}

diesel::joinable!(bans -> channels (channel_id));
diesel::joinable!(bans -> users (user_id));
diesel::joinable!(members -> channels (channel_id));
diesel::joinable!(members -> users (user_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bans,
    channels,
    members,
    messages,
    users,
);
