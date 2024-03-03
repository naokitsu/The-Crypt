// @generated automatically by Diesel CLI.

diesel::table! {
    secrets (user_id) {
        user_id -> Uuid,
        salted_hash -> Bytea,
    }
}

diesel::table! {
    sessions (id) {
        #[max_length = 72]
        id -> Varchar,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 64]
        username -> Varchar,
    }
}

diesel::joinable!(secrets -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    secrets,
    sessions,
    users,
);
