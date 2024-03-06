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
use crate::models::login_request::{LoginError, LoginRequest};
use crate::models::register_request::{RegisterError, RegisterRequest};
use crate::models::token::Token;
use crate::models::user::{User, UserError};


#[post("/login", format = "json", data = "<login_request>")]
pub(super) async fn login(login_request: Json<LoginRequest<'_>>, mut db: Connection<Db>) -> Result<Token, LoginError> {
    db.login(login_request.username, login_request.password).await
}

#[post("/register", format = "json", data = "<login_request>")]
pub(super) async fn register(login_request: Json<RegisterRequest<'_>>, mut db: Connection<Db>) -> Result<Token, RegisterError> {
    db.register(login_request.username, login_request.password).await
}

#[get("/me")]
pub(super) async fn me(user: Result<User, UserError>) -> Result<User, UserError> {
    // You can change the user's type into just `User`, but then an Unauthorized cather is going to be called
    // and i didn't write an own one
    user
}

