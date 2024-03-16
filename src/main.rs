extern crate core;
#[macro_use]
extern crate rocket;

use crate::auth::AuthService;
use crate::chat::ChatService;

mod auth;

pub mod schema;
mod models;
mod chat;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount_auth_service("/auth")
        .mount_chat_service("/chat")
}

