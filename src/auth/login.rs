use diesel::RunQueryDsl;
use rocket::serde::json::Json;
use crate::auth::objects::Error;
use crate::database;
use crate::schema::users;
use rocket::serde::Deserialize;
use rocket_db_pools::Connection;
use crate::database::{AuthDatabase, Db};
use rocket_db_pools::diesel::{prelude::*};
use uuid::Uuid;



#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[post("/login", format = "json", data = "<login_request>")]
pub(super) async fn login(login_request: Option<Json<LoginRequest<'_>>>, mut db: Connection<Db>) -> Result<String, Error> {
    use crate::schema::users::*;
    let login_request = login_request.ok_or(Error::BadRequest("Missing username or password".to_string()))?;

    let token = db.login(login_request.username, login_request.password).await.map_err(|err| match err {
        database::LoginError::NotFound => Error::Unauthorized("User not found".to_string()),
        database::LoginError::IncorrectPassword => Error::Unauthorized("Wrong password".to_string()),
        database::LoginError::InternalError => Error::InternalServerError("Internal error".to_string()),
    })?;

    Ok(token)
}

#[post("/register", format = "json", data = "<login_request>")]
pub(super) async fn register(login_request: Option<Json<LoginRequest<'_>>>, mut db: Connection<Db>) -> Result<String, Error> {
    use crate::schema::users::*;
    let login_request = login_request.ok_or(Error::BadRequest("Missing username or password".to_string()))?;

    let token = db.register(login_request.username, login_request.password).await.map_err(|err| match err {
        database::RegisterError::AlreadyInUse => Error::Unauthorized("User already in use".to_string()),
        database::RegisterError::InternalError => Error::InternalServerError("Internal error".to_string()),
    })?;

    Ok(token)
}

#[get("/me")]
pub(super) async fn me(user: database::User) -> Json<database::User> {
    Json(user)
}
