use diesel::result::Error;
use rocket::Rocket;
use rocket_db_pools::{Database};
use rocket_db_pools::diesel::{self, prelude::*};
use crate::schema;

const SALT_SIZE: usize = 16;
const HASH_SIZE: usize = 32;
const SALTED_SIZE: usize = SALT_SIZE + HASH_SIZE;

mod cryptography;


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


pub trait AuthDatabase {
    async fn login(&mut self, login: &str, password: &str) -> Result<String, LoginError>;
    async fn register(&mut self, login: &str, password: &str) -> Result<(), RegisterError>;
}

#[derive(Queryable, PartialEq, Debug)]
pub struct User{
    id: uuid::Uuid,
    username: String,
    salted_hash: Vec<u8>,
}

impl AuthDatabase for rocket_db_pools::Connection<Db> {
    async fn login(&mut self, login: &str, password: &str) -> Result<String, LoginError> {
        let user = schema::users::table
            .filter(schema::users::username.eq(login))
            .first::<User>(self)
            .await;

        match user {
            Ok(User{ salted_hash, .. }) => {
                let salt = &salted_hash[0..SALT_SIZE];
                let db_hash = &salted_hash[SALT_SIZE..];



                if cryptography::verify_password(salt, password.as_bytes(), db_hash).map_err(|_| LoginError::InternalError)? {
                    Ok(login.to_string())
                } else {
                    Err(LoginError::IncorrectPassword)
                }
            },
            Err(_) => Err(LoginError::NotFound),
        }
    }

    async fn register(&mut self, login: &str, password: &str) -> Result<(), RegisterError> {
        let user = schema::users::table
            .filter(schema::users::username.eq(login))
            .first::<User>(self)
            .await;

        match user {
            Err(_) => {
                let salted_hash = cryptography::hash_password(password.as_bytes()).map_err(|_| RegisterError::InternalError)?;
                let res = diesel::insert_into(schema::users::table)
                    .values((
                        schema::users::username.eq(login),
                        schema::users::salted_hash.eq(salted_hash),
                    ))
                    .execute(self)
                    .await
                    .map_err(|_| RegisterError::InternalError)?;

                if res == 1 {
                    Ok(())
                } else {
                    Err(RegisterError::InternalError)
                }
            },
            Ok(_) => Err(RegisterError::AlreadyInUse),
        }


    }
}

pub enum LoginError {
    NotFound,
    IncorrectPassword,
    InternalError,
}

pub enum RegisterError {
    AlreadyInUse,
    InternalError,
}
