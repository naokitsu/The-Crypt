use rocket::{Request};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use crate::database::auth_database::AuthDatabase;
use rocket_db_pools::Connection;
use crate::database::Db;
use crate::models::User;

#[derive(Debug)]
pub enum TokenLoginError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = TokenLoginError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        let mut connection: Connection<Db> = match request.guard::<Connection<Db>>().await {
            Outcome::Success(x) => x,
            Outcome::Error(_) => return Outcome::Error((Status::InternalServerError, TokenLoginError::Missing)),
            Outcome::Forward(_) => return Outcome::Forward(Status::InternalServerError), // TODO: Figure out this part
        };
        match token {
            Some(token) => {
                match connection.verify_login_token(token).await {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => Outcome::Error((Status::Unauthorized, TokenLoginError::Invalid)),
                }
            }
            None => Outcome::Error((Status::Unauthorized, TokenLoginError::Missing)),
        }
    }
}
