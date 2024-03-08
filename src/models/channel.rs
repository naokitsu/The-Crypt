use std::io::Write;
use std::str::FromStr;
use rocket::serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable, Queryable};
use rocket::data::{FromData, Outcome};
use rocket::{Data, Request};
use rocket::http::Status;
use rocket::request::FromParam;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::ser::SerializeStruct;
use crate::database::Db;
use crate::models::{ChannelId, LoginError, LoginRequest, RegisterRequest};
use crate::models::user::UserError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Channel {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Patch {
    pub id: Option<uuid::Uuid>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Insert {
    pub name: String,
}

#[async_trait]
impl<'r> Responder<'r, 'static> for Channel {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(_request)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotFound,
    Unauthorized,
    Forbidden,
    Conflict,
    InternalServerError,
}

#[async_trait]
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            Error::NotFound => {
                (Status::NotFound, Json(self)).respond_to(_request)
            },
            Error::Unauthorized => {
                (Status::Unauthorized, Json(self)).respond_to(_request)
            },
            Error::InternalServerError => {
                (Status::InternalServerError, Json(self)).respond_to(_request)
            },
            Error::Conflict => {
                (Status::Conflict, Json(self)).respond_to(_request)
            },
            Error::Forbidden => {
                (Status::Forbidden, Json(self)).respond_to(_request)
            },
        }
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("LoginError", 1)?;
        match self {
            Error::InternalServerError => state.serialize_field("message", "Could not login")?,
            Error::Unauthorized => state.serialize_field("message", "Incorrect username or password")?,
            Error::NotFound => state.serialize_field("message", "Channel not found")?,
            Error::Conflict => state.serialize_field("message", "Channel already exists")?,
            Error::Forbidden => state.serialize_field("message", "You are not an admin of the channel")?,
        }
        state.end()
    }
}

#[async_trait]
impl<'r> FromData<'r> for Patch {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        Json::from_data(req, data).await.map(|json: Json<Patch>| json.into_inner())
    }
}


#[async_trait]
impl<'r> FromData<'r> for Insert {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        Json::from_data(req, data).await.map(|json: Json<Insert>| json.into_inner())
    }
}
