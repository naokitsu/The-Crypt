// @generated automatically by Diesel CLI.

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
        salted_hash -> Bytea,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
