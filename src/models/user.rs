use diesel::Queryable;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use crate::database::{Db, AuthDatabase};
use rocket::serde::json::Json;
use serde::ser::SerializeStruct;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub is_admin: bool,
}

#[async_trait]
impl<'r> Responder<'r, 'static> for User {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(request)
    }
}

#[derive(Debug)]
pub enum UserError {
    InternalServerError,
    Unauthorized,
}

#[async_trait]
impl<'r> Responder<'r, 'static> for UserError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            UserError::InternalServerError => {
                (Status::InternalServerError, Json(self)).respond_to(_request)
            },
            UserError::Unauthorized => {
                (Status::Unauthorized, Json(self)).respond_to(_request)
            },
        }
    }
}

impl Serialize for UserError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer
    {
        let mut state = serializer.serialize_struct("LoginError", 1)?;
        match self {
            UserError::InternalServerError => state.serialize_field("message", "Could not login")?,
            UserError::Unauthorized => state.serialize_field("message", "Incorrect access token")?,
        }
        state.end()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = UserError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        let mut connection: Connection<Db> = match request.guard::<Connection<Db>>().await {
            Outcome::Success(x) => x,
            _ => return Outcome::Error((Status::InternalServerError, UserError::InternalServerError)),
        };
        match token {
            Some(token) => {
                let token = token.trim_start_matches("BEARER ");
                let mut fixed_array = [0u8; 32];
                fixed_array.copy_from_slice(token.as_bytes());
                todo!("Token Verification Here, it shall return the user")
                /*match connection.verify_login_token(fixed_array).await {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => Outcome::Error((Status::Unauthorized, UserError::Unauthorized)),
                }*/
            }
            None => Outcome::Error((Status::Unauthorized, UserError::Unauthorized)),
        }
    }
}