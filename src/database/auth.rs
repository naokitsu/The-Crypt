use diesel::result::Error;

use crate::database::Db;
use crate::database::token::Database;
use crate::models::{LoginError, RegisterError, Token};

pub(crate) trait AuthDatabase<T: super::token::Database = Self> {
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
    async fn login(&mut self, login: &str, _password: &str) -> Result<Token, LoginError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::users;

        let db_id = users::table
            .select(users::id)
            .filter(users::username.eq(login))
            .first::<uuid::Uuid>(self)
            .await
            .map_err(|err| match err {
                Error::NotFound => LoginError::Unauthorized,
                _ => LoginError::InternalServerError
            })?;

        let mut key: [u8; 32] = [0; 32];
        let _nonce: [u8; 32] = [0; 32];
        unsafe { // TODO
            std::ptr::copy(login.as_ptr(), key.as_mut_ptr(), std::cmp::min(login.len(), 32));
        }
        let db_key = self.get_public_key(db_id)
            .await
            .map_err(|_| LoginError::InternalServerError)?;

        if db_key == key {
            // Todo It all should not look like this, i hope lyly's verification will go there one day
            Ok(Token { access_token: login.to_string() })
        } else {
            Err(LoginError::Unauthorized)
        }
    }

    async fn register(&mut self, login: &str, _password: &str) -> Result<Token, RegisterError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::users;

        let db_id = rocket_db_pools::diesel::insert_into(users::table)
            .values((
                users::username.eq(login),
            ))
            .returning(users::id)
            .get_result::<uuid::Uuid>(self)
            .await
            .map_err(|err| match err {
                Error::DatabaseError(rocket_db_pools::diesel::result::DatabaseErrorKind::UniqueViolation, _) =>
                    RegisterError::Conflict,
                _ => RegisterError::InternalServerError,
            })?;

        let mut key: [u8; 32] = [0; 32];
        let nonce: [u8; 32] = [0; 32];
        unsafe { // TODO
            std::ptr::copy(login.as_ptr(), key.as_mut_ptr(), std::cmp::min(login.len(), 32));
        }
        self.insert_session(db_id, key, nonce)
            .await
            .map_err(|_| RegisterError::InternalServerError)?;
        Ok(Token { access_token: login.to_string() })
    }
}

