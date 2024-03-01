use rocket_db_pools::diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub salted_hash: Vec<u8>,
}