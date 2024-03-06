// @generated automatically by Diesel CLI.

diesel::table! {
    secrets (user_id) {
        user_id -> Uuid,
        salted_hash -> Bytea,
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

diesel::joinable!(secrets -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    secrets,
    sessions,
    users,
);
