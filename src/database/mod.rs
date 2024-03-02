use std::io::Write;
use diesel::result::Error;
use diesel::RunQueryDsl;
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
    async fn register(&mut self, login: &str, password: &str) -> Result<String, RegisterError>;
    async fn generate_login_token(&mut self, user: uuid::Uuid) -> Result<String, TokenError>;
}

#[derive(Queryable, PartialEq, Debug)]
pub struct User{
    id: uuid::Uuid,
    username: String,
    salted_hash: Vec<u8>,
}

impl AuthDatabase for rocket_db_pools::Connection<Db> {
    async fn login(&mut self, login: &str, password: &str) -> Result<String, LoginError> {
        let (user_salted_hash, user_uuid) = schema::users::table
            .select((schema::users::salted_hash, schema::users::id))
            .filter(schema::users::username.eq(login))
            .first::<(Vec<u8>, uuid::Uuid)>(self)
            .await
            .map_err(|_| LoginError::NotFound)?;

        let salt = &user_salted_hash[0..SALT_SIZE];
        let db_hash = &user_salted_hash[SALT_SIZE..];

        if cryptography::verify_password(salt, password.as_bytes(), db_hash).map_err(|_| LoginError::InternalError)? {
            let login_token = self.generate_login_token(user_uuid).await.map_err(|_| LoginError::InternalError)?;
            Ok(login_token)
        } else {
            Err(LoginError::IncorrectPassword)
        }
    }

    async fn register(&mut self, login: &str, password: &str) -> Result<String, RegisterError> {
        let user_count = schema::users::table
            .filter(schema::users::username.eq(login))
            .count()
            .get_result::<i64>(self)
            .await
            .map_err(|_| RegisterError::InternalError)?;

        if user_count > 0 {
            return Err(RegisterError::AlreadyInUse);
        }

        let salted_hash = cryptography::hash_password(password.as_bytes()).map_err(|_| RegisterError::InternalError)?;
        let res = diesel::insert_into(schema::users::table)
            .values((
                schema::users::username.eq(login),
                schema::users::salted_hash.eq(salted_hash),
            ))
            .execute(self)
            .await
            .map_err(|_| RegisterError::InternalError)?;

        let user_uuid = schema::users::table
            .select(schema::users::id)
            .filter(schema::users::username.eq(login))
            .first::<uuid::Uuid>(self)
            .await
            .map_err(|_| RegisterError::InternalError)?;

        if res == 1 {
            let login_token = self.generate_login_token(user_uuid).await.map_err(|_| RegisterError::InternalError)?;
            Ok(login_token)
        } else {
            Err(RegisterError::InternalError)
        }
    }

    async fn generate_login_token(&mut self, user_uuid: uuid::Uuid) -> Result<String, TokenError> {
        let login_token = cryptography::gen_login_token().map_err(|_| TokenError::InternalError)?;

        let res = diesel::insert_into(schema::sessions::table)
            .values((
                schema::sessions::id.eq(&login_token),
                schema::sessions::user_id.eq(user_uuid),
            ))
            .execute(self)
            .await
            .map_err(|_| TokenError::InternalError)?;

        if res == 1 {
            Ok(login_token)
        } else {
            Err(TokenError::InternalError)
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

enum TokenError {
    InternalError,
}