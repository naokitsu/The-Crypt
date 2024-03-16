use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::impl_from_data_json_for;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct RegisterRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl_from_data_json_for!(RegisterRequest<'a>);

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InternalServerError,
    Unauthorized,
}

#[async_trait]
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            Error::InternalServerError => {
                (Status::InternalServerError, Json(self)).respond_to(req)
            }
            Error::Unauthorized => {
                (Status::Unauthorized, Json(self)).respond_to(req)
            }
        }
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Error", 1)?;
        match self {
            Error::InternalServerError => state.serialize_field("message", "Could not register")?,
            Error::Unauthorized => state.serialize_field("message", "Incorrect username or password")?,
        }
        state.end()
    }
}