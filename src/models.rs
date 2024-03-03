use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub salted_hash: Vec<u8>,
}

