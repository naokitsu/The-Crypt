use std::fmt::Display;
use rocket::http::uri::Origin;
use rocket::response::Redirect;
use rocket_contrib::databases::diesel::sql_types::Json;
use rocket_db_pools::Connection;
use crate::database::Db;
use crate::models;
use crate::database::auth::AuthDatabase;
use crate::models::{AuthClaims, LoginError, RegisterError, Token};

pub(crate) trait Auth {
    fn mount_auth<'a, B>(self, base: B) -> Self where B: TryInto<Origin<'a>> + Clone + Display, B::Error: Display;
}

impl Auth for rocket::Rocket<rocket::Build> {
    fn mount_auth<'a, B>(self, base: B) -> Self where B: TryInto<Origin<'a>> + Clone + Display, B::Error: Display {
        self.mount(base, routes![login, register, ping])
    }
}

#[post("/login", format = "json", data = "<login_request>")]
pub async fn login(login_request: models::LoginRequest<'_>, mut db: Connection<Db>) -> Result<models::Token, LoginError> {
    db.login(login_request.username, login_request.password).await
}

#[post("/register", format = "json", data = "<register_request>")]
pub async fn register(register_request: models::RegisterRequest<'_>, mut db: Connection<Db>) -> Result<Token, RegisterError> {
    db.register(register_request.username, register_request.password).await?;
    db.login(register_request.username, register_request.password).await.map_err(|e| match e {
        LoginError::InternalServerError | LoginError::Unauthorized => RegisterError::InternalServerError
        // LoginError::Unauthorized shouldn't happen, user has been registered one line before calling this
        // but there may be a place for the race condition, so it should return InternalServerError too
    })
}

#[get("/ping")]
fn ping(claims: models::AuthClaims) -> String {
    format!("{:?}", claims)
}

