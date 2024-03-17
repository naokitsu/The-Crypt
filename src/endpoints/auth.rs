use std::fmt::Display;
use rocket::http::uri::Origin;
use rocket_contrib::databases::diesel::sql_types::Json;
use rocket_db_pools::Connection;
use crate::database::Db;
use crate::models;
use crate::database::auth::AuthDatabase;

pub(crate) trait Auth {
    fn mount_auth<'a, B>(self, base: B) -> Self where B: TryInto<Origin<'a>> + Clone + Display, B::Error: Display;
}

impl Auth for rocket::Rocket<rocket::Build> {
    fn mount_auth<'a, B>(self, base: B) -> Self where B: TryInto<Origin<'a>> + Clone + Display, B::Error: Display {
        self.mount(base, routes![login, register])
    }
}

#[post("/login", format = "json", data = "<login_request>")]
pub async fn login(login_request: models::LoginRequest<'_>, mut db: Connection<Db>) -> Result<models::Token, ()> {
    db.login(login_request.username, login_request.password).await
}

#[post("/register", format = "json", data = "<login_request>")]
pub async fn register(login_request: models::RegisterRequest<'_>, mut db: Connection<Db>) -> Result<(), ()> {
    db.register(login_request.username, login_request.password).await
}

