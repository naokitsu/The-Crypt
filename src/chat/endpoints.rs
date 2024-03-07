use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::database::{Db, AuthDatabase};
use crate::models;


#[post("/login", format = "json", data = "<login_request>")]
pub async fn login(login_request: models::LoginRequest<'_>, mut db: Connection<Db>) -> Result<models::Token, models::LoginError> {
    db.login(login_request.username, login_request.password).await
}

#[post("/register", format = "json", data = "<login_request>")]
pub async fn register(login_request: models::RegisterRequest<'_>, mut db: Connection<Db>) -> Result<models::Token, models::RegisterError> {
    db.register(login_request.username, login_request.password).await
}

#[get("/me")]
pub async fn me(user: Result<models::user::User, models::user::UserError>) -> Result<models::user::User, models::user::UserError> {
    // You can change the user's type into just `User`, but then an Unauthorized cather is going to be called
    // and i didn't write an own one
    user
}

