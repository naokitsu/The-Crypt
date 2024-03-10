use rocket::data::{FromData, Outcome};
use rocket::http::Status;
use rocket::{Data, Request};
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use serde::ser::SerializeStruct;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct RegisterRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[async_trait]
impl<'r> FromData<'r> for RegisterRequest<'r> {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        Json::from_data(req, data).await.map(|json: Json<RegisterRequest>| json.into_inner())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum RegisterError {
    InternalServerError,
    Conflict,
}

#[async_trait]
impl<'r> Responder<'r, 'static> for RegisterError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        // match with respective error codes and json body
        match self {

            RegisterError::InternalServerError => {
                (Status::InternalServerError, Json(self)).respond_to(_request)
            },
            RegisterError::Conflict => {
                (Status::Conflict, Json(self)).respond_to(_request)
            },
        }
    }
}

impl Serialize for RegisterError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("RegisterError", 1)?;
        match self {
            RegisterError::InternalServerError => state.serialize_field("message", "Could not register")?,
            RegisterError::Conflict => state.serialize_field("message", "The username is already in use")?,
        }
        state.end()
    }
}



