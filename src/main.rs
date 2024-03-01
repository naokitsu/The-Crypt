#[macro_use] extern crate rocket;

use crate::auth::AuthService;
use crate::database::PostgreSQLDatabase;

mod auth;
mod database;
pub mod schema;


#[launch]
fn rocket() -> _ {

    rocket::build()
        .attach_database()
        .mount_auth_service("/auth")
}

