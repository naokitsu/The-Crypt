use diesel::deserialize::FromSqlRow;
use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel::Queryable;
use rocket_db_pools::{diesel, Database};
mod auth;
pub(crate) mod token;
pub(crate) mod channels;

pub(crate) use auth::AuthDatabase;

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


