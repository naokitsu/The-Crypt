use diesel::result::Error;
use crate::database::{auth_database, cryptography, Db};
use crate::models::{LoginError, RegisterError, Token, User};

pub(crate) trait AuthDatabase<T: super::token_database::Database = Self> {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, LoginError>;
    async fn register(&mut self, login: &str, password: &str) -> Result<Token, RegisterError>;
}

pub enum TokenGenerateError {
    InternalServerError,
}

pub enum TokenVerificationError {
    InternalServerError,
    Unauthorized,
}

impl From<TokenGenerateError> for LoginError {
    fn from(value: TokenGenerateError) -> Self {
        match value {
            TokenGenerateError::InternalServerError =>
                LoginError::InternalServerError,

        }
    }
}

impl From<TokenVerificationError> for LoginError {
    fn from(value: TokenVerificationError) -> Self {
        match value {
            TokenVerificationError::InternalServerError =>
                LoginError::InternalServerError,
            TokenVerificationError::Unauthorized =>
                LoginError::Unauthorized,
        }
    }
}

impl AuthDatabase for rocket_db_pools::Connection<Db> {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, LoginError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::{users, secrets};

        let (db_salt, db_id) = users::table
            .inner_join(secrets::table)
            .select((secrets::salt, users::id))
            .filter(users::username.eq(login))
            .first::<(Vec<u8>, uuid::Uuid)>(self)
            .await
            .map_err(|err| match err {
                Error::NotFound => LoginError::Unauthorized,
                _ => LoginError::InternalServerError
            })?;

        let _ = db_salt; // salt
        let _ = db_id; // pepper

        if true {
            todo!("Token Generation Here")
        } else {
            Err(LoginError::Unauthorized)
        }

    }

    async fn register(&mut self, login: &str, password: &str) -> Result<Token, RegisterError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::{users, secrets};

        let (db_id, db_is_admin) = rocket_db_pools::diesel::insert_into(users::table)
            .values((
                users::username.eq(login),
            ))
            .returning((users::id, users::is_admin))
            .get_result::<(uuid::Uuid, bool)>(self)
            .await
            .map_err(|err| match err {
                Error::DatabaseError(rocket_db_pools::diesel::result::DatabaseErrorKind::UniqueViolation, _) =>
                    RegisterError::Conflict,
                _ => RegisterError::InternalServerError,
            })?;

        rocket_db_pools::diesel::insert_into(secrets::table)
            .values((
                secrets::user_id.eq(db_id),
                secrets::salt.eq(todo!("Salt generate")),
            ))
            .execute(self)
            .await
            .map_err(|_| RegisterError::InternalServerError)?;

        todo!("Token Generation Here")
    }
}

