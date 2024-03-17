extern crate core;
#[macro_use]
extern crate rocket;

use crate::database::PostgreSQLDatabase;
use crate::endpoints::Auth;


pub mod schema;
mod models;
mod chat;
mod endpoints;
mod database;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach_database()
        .mount_auth("/auth")
//        .mount_auth_service("/auth")
//        .mount_chat_service("/chat")
}

