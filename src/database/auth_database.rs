use diesel::result::Error;
use rocket::http::Status;
use crate::database::cryptography::SALT_SIZE;
use crate::database::Db;
use crate::models::login_request::{LoginError};
use crate::models::register_request::RegisterError;
use crate::models::token::Token;
use crate::models::user::User;
use super::cryptography;
use crate::schema;
use crate::schema::sessions;
use crate::schema::users::is_admin;

pub(crate) trait AuthDatabase {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, LoginError>;
    async fn register(&mut self, login: &str, password: &str) -> Result<Token, RegisterError>;
    async fn generate_token(&mut self, user_id: uuid::Uuid, is_admin: bool) -> Result<Token, TokenGenerateError>;
    async fn verify_login_token(&mut self, login_token: &str) -> Result<User, Status>;
}

enum TokenGenerateError {
    InternalServerError
}

impl From<TokenGenerateError> for LoginError {
    fn from(value: TokenGenerateError) -> Self {
        match value {
            TokenGenerateError::InternalServerError =>
                LoginError::InternalServerError,
        }
    }
}

impl AuthDatabase for rocket_db_pools::Connection<Db> {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, LoginError> {
        use rocket_db_pools::diesel::prelude::*;
        use schema::{users, secrets};

        let (user_salted_hash, user_id, user_role) = users::table
            .inner_join(secrets::table)
            .select((secrets::salted_hash, users::id, users::is_admin))
            .filter(users::username.eq(login))
            .first::<(Vec<u8>, uuid::Uuid, bool)>(self)
            .await
            .map_err(|err| match err {
                Error::NotFound => LoginError::Unauthorized,
                _ => LoginError::InternalServerError
            })?;

        if user_salted_hash.len() <= SALT_SIZE { return Err(LoginError::InternalServerError) }
        let (salt, hash) = user_salted_hash
            .split_at(SALT_SIZE);

        if cryptography::verify_password(salt, password.as_bytes(), hash).map_err(|_| LoginError::InternalServerError)? {
            self.generate_token(user_id, user_role).await.map_err(|err| err.into())
        } else {
            Err(LoginError::Unauthorized)
        }

    }

    async fn register(&mut self, login: &str, password: &str) -> Result<Token, RegisterError> {
        use rocket_db_pools::diesel::prelude::*;
        use schema::{users, secrets};

        let salt_hash = cryptography::hash_password(password.as_bytes())
            .map_err(|_| RegisterError::InternalServerError)?;

        let (user_uuid, user_role) = diesel::insert_into(users::table)
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
                secrets::user_id.eq(user_uuid),
                secrets::salted_hash.eq(salt_hash),
            ))
            .execute(self)
            .await
            .map_err(|_| RegisterError::InternalServerError)?;

        self
            .generate_token(user_uuid, user_role)
            .await
            .map_err(|_| RegisterError::InternalServerError)
    }



    async fn generate_token(&mut self, user_id_: uuid::Uuid, is_admin_: bool) -> Result<Token, TokenGenerateError> {
        use rocket_db_pools::diesel::prelude::*;
        use schema::sessions::{self, dsl::*};

        let token = cryptography::gen_login_token(user_id_, is_admin_)
            .map_err(|_| TokenGenerateError::InternalServerError)?;

        let lines_affected = diesel::insert_into(sessions::table)
            .values((
                id.eq(&token),
                user_id.eq(user_id_),
            ))
            .execute(self)
            .await
            .map_err(|_| TokenGenerateError::InternalServerError)?;

        if lines_affected == 1 {
            Ok(Token{ access_token: token })
        } else {
            Err(TokenGenerateError::InternalServerError)
        }
    }

    async fn verify_login_token(&mut self, login_token: &str) -> Result<User, Status> {
        use rocket_db_pools::diesel::prelude::*;
        use schema::sessions::{self};
        use schema::users::{self};

        users::table
            .inner_join(sessions::table)
            .filter(sessions::id.eq(login_token))
            .select(users::all_columns)
            .first::<User>(self)
            .await
            .map_err(|x| match x {
                Error::NotFound => Status::NotFound,
                _ => Status::InternalServerError,
            })
    }
}
