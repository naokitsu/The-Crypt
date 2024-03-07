use rocket::data::{FromData, Outcome};
use rocket::http::Status;
use rocket::{Data, Request};
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use serde::ser::SerializeStruct;
use crate::models::RegisterRequest;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[async_trait]
impl<'r> FromData<'r> for LoginRequest<'r> {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        Json::from_data(req, data).await.map(|json: Json<LoginRequest>| json.into_inner())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoginError {
    InternalServerError,
    Unauthorized,
}

#[async_trait]
impl<'r> Responder<'r, 'static> for LoginError {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            LoginError::InternalServerError => {
                (Status::InternalServerError, Json(self)).respond_to(_request)
            },
            LoginError::Unauthorized => {
                (Status::Unauthorized, Json(self)).respond_to(_request)
            },
        }
    }
}

impl Serialize for LoginError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("LoginError", 1)?;
        match self {
            LoginError::InternalServerError => state.serialize_field("message", "Could not login")?,
            LoginError::Unauthorized => state.serialize_field("message", "Incorrect username or password")?,
        }
        state.end()
    }
}