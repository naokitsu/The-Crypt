use diesel::result::Error;
use crate::database::{auth_database, cryptography, Db};
use crate::models::{LoginError, RegisterError, Token, User};

pub(crate) trait AuthDatabase<T: super::token_database::Database = Self> {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, LoginError>;
    async fn register(&mut self, login: &str, password: &str) -> Result<Token, RegisterError>;
    async fn generate_token(&mut self, user_id: uuid::Uuid, is_admin: bool) -> Result<Token, TokenGenerateError>;
    async fn verify_login_token(&mut self, login_token: [u8; 32]) -> Result<User, TokenVerificationError>;
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

        let (db_id, db_is_admin) = rocket_db_pools::diesel::insert_into(users::table)
            .values((
                users::username.eq(login),
            ))
            .returning((users::id, users::is_admin))
            .get_result(self)
            .await
            .map_err(|err| match err {
                Error::DatabaseError(rocket_db_pools::diesel::result::DatabaseErrorKind::UniqueViolation, _) =>
                    RegisterError::Conflict,
                _ => RegisterError::InternalServerError,
            })?;

        rocket_db_pools::diesel::insert_into(secrets::table)
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
        let mut key_random: [u8; 32] = [0; 32];
        let mut nonce_random: [u8; 32] = [0; 32];
        openssl::rand::rand_bytes(&mut key_random).and_then(
            |_| openssl::rand::rand_bytes(&mut nonce_random)
        ).map_err(|_| auth_database::TokenGenerateError::InternalServerError)?;

        self.insert_session(db_id, key_random, nonce_random, db_is_admin)
            .await
            .map_err(|_| auth_database::TokenGenerateError::InternalServerError)?;

        Ok(Token{ access_token: "Placeholder".to_string() })
    }

    async fn verify_login_token(&mut self, login_token: [u8; 32]) -> Result<User, TokenVerificationError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::sessions::{self};
        use crate::schema::users::{self};

        if let Err(err) = cryptography::verify_login_token(&login_token) {
            return match err {
                cryptography::TokenVerifyError::Expired => {
                    if let Err(_) = rocket_db_pools::diesel::delete(sessions::table)
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

