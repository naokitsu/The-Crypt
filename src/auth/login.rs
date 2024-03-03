use diesel::RunQueryDsl;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::database;
use crate::schema::users;
use rocket::serde::Deserialize;
use rocket_db_pools::Connection;
use crate::database::{auth_database::AuthDatabase, Db};
use rocket_db_pools::diesel::{prelude::*};
use uuid::Uuid;
use crate::models::User;


#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[post("/login", format = "json", data = "<login_request>")]
pub(super) async fn login(login_request: Option<Json<LoginRequest<'_>>>, mut db: Connection<Db>) -> Result<String, Status> {
    use crate::schema::users::*;
    let login_request = login_request.ok_or(Status::BadRequest)?;

    db.login(login_request.username, login_request.password).await
}

#[post("/register", format = "json", data = "<login_request>")]
pub(super) async fn register(login_request: Option<Json<LoginRequest<'_>>>, mut db: Connection<Db>) -> Result<String, Status> {
    use crate::schema::users::*;
    let login_request = login_request.ok_or(Status::BadRequest)?;

    db.register(login_request.username, login_request.password).await
}

#[get("/me")]
pub(super) async fn me(user: User) -> Json<User> {
    Json(user)
}
