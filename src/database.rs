use rocket_db_pools::{Database};
use rocket_db_pools::diesel::{PgPool, prelude::*};

#[derive(Database)]
#[database("chat_app")]
struct DatabaseConnection (PgPool);

pub trait PostgreSQLDatabase {
    fn attach_database(self) -> Self;
}
/*
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

async fn run_migrations(mut rocket: Rocket<Build>) -> Result<(), ()> {
    if let Some(con) = DatabaseConnection::fetch(&mut rocket) {

        if let Err(e) = con.run_pending_migrations(MIGRATIONS) {
            eprint!("Failed to run migrations: {}", e);
            return Err(());
        }

    } else {
        eprint!("Failed to get database connection for migrations");
        return Err(());
    }
    return Ok(());
}
*/

impl PostgreSQLDatabase for rocket::Rocket<rocket::Build> {
    fn attach_database(self) -> Self {
        self.attach(DatabaseConnection::init())
            //.attach(rocket::fairing::AdHoc::on_ignite("Database migrations", run_migrations))
    }
}