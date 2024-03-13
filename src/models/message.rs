use diesel::{Insertable, Queryable};
use rocket::Request;
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use crate::models::Channel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub channel_id: uuid::Uuid,
    pub content: String,
}

pub struct Patch {
    pub content: Option<String>,
}

pub struct Insert {
    pub user_id: uuid::Uuid,
    pub channel_id: uuid::Uuid,
    pub content: String,
}

pub struct Error;

#[async_trait]
impl<'r> Responder<'r, 'static> for Message {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(_request)
    }
}