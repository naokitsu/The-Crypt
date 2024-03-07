#[macro_use] extern crate rocket;
extern crate core;

use crate::auth::AuthService;
use crate::database::PostgreSQLDatabase;


mod auth;

pub mod schema;
mod models;
mod database;
mod chat;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach_database()
        .mount_auth_service("/auth")
}

