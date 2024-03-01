// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 8]
        salt -> Varchar,
        #[max_length = 16]
        password_hash -> Varchar,
    }
}
