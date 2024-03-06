use diesel::deserialize::FromSqlRow;
use diesel::result::Error;
use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel::Queryable;
use rocket_db_pools::{diesel, Database};
mod cryptography;
mod auth_database;
mod token_database;

pub(crate) use auth_database::AuthDatabase;
use crate::database::auth_database::TokenVerificationError;
use crate::models::{LoginError, RegisterError, Token, User};

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

impl AuthDatabase for rocket_db_pools::Connection<Db> {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, LoginError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::{users, secrets};

        let (db_salted_hash, db_id, db_is_admin) = users::table
            .inner_join(secrets::table)
            .select((secrets::salted_hash, users::id, users::is_admin))
            .filter(users::username.eq(login))
            .first::<(Vec<u8>, uuid::Uuid, bool)>(self)
            .await
            .map_err(|err| match err {
                Error::NotFound => LoginError::Unauthorized,
                _ => LoginError::InternalServerError
            })?;

        if db_salted_hash.len() <= cryptography::SALT_SIZE { return Err(LoginError::InternalServerError) }
        let (salt, hash) = db_salted_hash
            .split_at(cryptography::SALT_SIZE);

        if cryptography::verify_password(salt, password.as_bytes(), hash).map_err(|_| LoginError::InternalServerError)? {
            self.generate_token(db_id, db_is_admin).await.map_err(|err| err.into())
        } else {
            Err(LoginError::Unauthorized)
        }

    }

    async fn register(&mut self, login: &str, password: &str) -> Result<Token, RegisterError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::{users, secrets};

        let salt_hash = cryptography::hash_password(password.as_bytes())
            .map_err(|_| RegisterError::InternalServerError)?;

        let (db_id, db_is_admin) = diesel::insert_into(users::table)
            .values((
                users::username.eq(login),
            ))
            .returning((users::id, users::is_admin))
            .get_result(self)
            .await
            .map_err(|err| match err {
                Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) =>
                    RegisterError::Conflict,
                _ => RegisterError::InternalServerError,
            })?;

        diesel::insert_into(secrets::table)
            .values((
                secrets::user_id.eq(db_id),
                secrets::salted_hash.eq(salt_hash),
            ))
            .execute(self)
            .await
            .map_err(|_| RegisterError::InternalServerError)?;

        self
            .generate_token(db_id, db_is_admin)
            .await
            .map_err(|_| RegisterError::InternalServerError)
    }

    async fn generate_token(&mut self, db_id: uuid::Uuid, db_is_admin: bool) -> Result<Token, auth_database::TokenGenerateError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::sessions::{self, dsl::*};

        let mut key_random: [u8; 32] = [0; 32];
        let mut nonce_random: [u8; 32] = [0; 32];
        openssl::rand::rand_bytes(&mut key_random).and_then(
            |_| openssl::rand::rand_bytes(&mut nonce_random)
        ).map_err(|_| auth_database::TokenGenerateError::InternalServerError)?;

        let lines_affected = diesel::insert_into(sessions::table)
            .values((
                sessions::key.eq(&key_random.as_slice()),
                sessions::nonce.eq(&nonce_random.as_slice()),
                sessions::user_id.eq(db_id),
            ))
            .execute(self)
            .await
            .map_err(|_| auth_database::TokenGenerateError::InternalServerError)?;

        if lines_affected == 1 {
            Ok(Token{ access_token: "Placeholder".to_string() })
        } else {
            Err(auth_database::TokenGenerateError::InternalServerError)
        }
    }

    async fn verify_login_token(&mut self, login_token: [u8; 32]) -> Result<User, TokenVerificationError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::sessions::{self};
        use crate::schema::users::{self};

        if let Err(err) = cryptography::verify_login_token(&login_token) {
            return match err {
                cryptography::TokenVerifyError::Expired => {
                    if let Err(_) = diesel::delete(sessions::table)
                        .filter(sessions::key.eq(login_token.as_slice()))
                        .execute(self)
                        .await {
                        Err(TokenVerificationError::InternalServerError)
                    } else {
                        Err(TokenVerificationError::Unauthorized)
                    }
                },
                _ => Err(TokenVerificationError::InternalServerError),
            };
        }



        users::table
            .inner_join(sessions::table)
            .filter(sessions::key.eq(login_token.as_slice()))
            .select(users::all_columns)
            .first::<User>(self)
            .await
            .map_err(|x| match x {
                Error::NotFound => TokenVerificationError::Unauthorized,
                _ => TokenVerificationError::InternalServerError,
            })
    }
}
