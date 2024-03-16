pub(crate) use auth::AuthDatabase;
use rocket_db_pools::{Database, diesel};

mod auth;
pub(crate) mod token;
pub(crate) mod channels;

#[derive(Database)]
#[database("chat_app")]
pub struct Db(diesel::PgPool);

pub trait PostgreSQLDatabase {
    fn attach_database(self) -> Self;
}

impl PostgreSQLDatabase for rocket::Rocket<rocket::Build> {
    fn attach_database(self) -> Self {
        self.attach(Db::init())
        // TODO: migrations
    }
}


