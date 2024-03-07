// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Uuid,
        #[max_length = 32]
        name -> Varchar,
        admin_id -> Uuid,
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
    users (id) {
        id -> Uuid,
        #[max_length = 64]
        username -> Varchar,
        is_admin -> Bool,
    }
}

diesel::joinable!(channels -> users (admin_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(secrets -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    messages,
    secrets,
    sessions,
    users,
);
