use diesel::Queryable;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::ser::SerializeStruct;

use crate::database::Db;
use crate::database::token::Database;
use crate::models::LoginError;

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
                let token = token;
                let mut key: [u8; 32] = [0; 32];
                let _nonce: [u8; 32] = [0; 32];
                unsafe { // TODO
                    std::ptr::copy(token.as_ptr(), key.as_mut_ptr(), std::cmp::min(token.len(), 32));
                }

                // ------ It is so cursed ---------
                use rocket_db_pools::diesel::prelude::*;
                use crate::schema::users;

                let user = users::table
                    .filter(users::username.eq(token))
                    .first::<User>(&mut connection)
                    .await;

                let user = match user {
                    Ok(x) => x,
                    Err(_) => return Outcome::Error((Status::Unauthorized, UserError::Unauthorized)),
                };

                let db_key = connection.get_public_key(user.id)
                    .await
                    .map_err(|_| LoginError::InternalServerError);

                let db_key = match db_key {
                    Ok(x) => x,
                    Err(_) => return Outcome::Error((Status::InternalServerError, UserError::InternalServerError)),
                };

                match db_key == key {
                    true => Outcome::Success(user),
                    false => Outcome::Error((Status::Unauthorized, UserError::Unauthorized)),
                }
            }
            None => Outcome::Error((Status::Unauthorized, UserError::Unauthorized)),
        }
    }
}