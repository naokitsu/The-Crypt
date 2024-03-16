use diesel::{AsChangeset, Insertable, Queryable};
use rocket::{Data, Request};
use rocket::data::{FromData, Outcome};
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Patch {
    pub content: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Insert {
    pub content: String,
}

pub struct Error;

#[async_trait]
impl<'r> Responder<'r, 'static> for Message {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(_request)
    }
}

#[async_trait]
impl<'r> FromData<'r> for Insert {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
    }
}


#[async_trait]
impl<'r> FromData<'r> for Patch {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
    }
}
