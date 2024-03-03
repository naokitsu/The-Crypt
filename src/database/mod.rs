use std::io::Write;
use diesel::RunQueryDsl;
use rocket::Rocket;
use rocket_db_pools::{Database};
use rocket_db_pools::diesel::{self, prelude::*};
use serde::Serialize;
use auth_database::AuthDatabase;

mod cryptography;
pub(crate) mod auth_database;

#[derive(Database)]
#[database("chat_app")]
pub struct Db(diesel::PgPool);

pub trait PostgreSQLDatabase {
    fn attach_database(self) -> Self;
}

impl PostgreSQLDatabase for Rocket<rocket::Build> {
    fn attach_database(self) -> Self {
        self.attach(Db::init())
        // TODO: migrations
    }
}


