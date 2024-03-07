use diesel::deserialize::FromSqlRow;
use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel::Queryable;
use rocket_db_pools::{diesel, Database};
mod auth_database;
pub(crate) mod token_database;

pub(crate) use auth_database::AuthDatabase;

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


