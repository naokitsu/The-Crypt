#[macro_use] extern crate rocket;

use crate::auth::AuthService;

mod auth;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount_auth_service("/auth")
}

