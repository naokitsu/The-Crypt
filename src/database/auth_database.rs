use diesel::result::Error;
use rocket::http::Status;
use crate::database::cryptography::SALT_SIZE;
use crate::database::Db;
use crate::models::User;
use super::cryptography;
use crate::schema;

pub(crate) trait AuthDatabase {
    async fn login(&mut self, login: &str, password: &str) -> Result<String, Status>;
    async fn register(&mut self, login: &str, password: &str) -> Result<String, Status>;
    async fn generate_login_token(&mut self, user: uuid::Uuid) -> Result<String, Status>;
    async fn verify_login_token(&mut self, login_token: &str) -> Result<User, Status>;
}


impl AuthDatabase for rocket_db_pools::Connection<Db> {
    async fn login(&mut self, login: &str, password: &str) -> Result<String, Status> {
        use rocket_db_pools::diesel::prelude::*;
        use schema::users::{self, dsl::*};

        let (user_salted_hash, user_uuid) = users::table
            .select((salted_hash, id))
            .filter(username.eq(login))
            .first::<(Vec<u8>, uuid::Uuid)>(self)
            .await
            .map_err(|x| match x {
                Error::NotFound => Status::NotFound,
                _ => Status::InternalServerError,
            })?;


        if user_salted_hash.len() <= SALT_SIZE { return Err(Status::InternalServerError) }
        let (salt, hash) = user_salted_hash
            .split_at(SALT_SIZE);

        if cryptography::verify_password(salt, password.as_bytes(), hash).map_err(|_| Status::InternalServerError)? {
            self.generate_login_token(user_uuid).await
        } else {
            Err(Status::Unauthorized)
        }
    }

    async fn register(&mut self, login: &str, password: &str) -> Result<String, Status> {
        use rocket_db_pools::diesel::prelude::*;
        use schema::users::{self, dsl::*};

        let salt_hash = cryptography::hash_password(password.as_bytes())
            .map_err(|_| Status::InternalServerError)?;

        let user_uuid = diesel::insert_into(users::table)
            .values((
                username.eq(login),
                salted_hash.eq(salt_hash),
            ))
            .returning(id)
            .get_result(self)
            .await
            .map_err(|err| match err {
                Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) =>
                    Status::Conflict,
                _ => Status::InternalServerError,
            })?;

        self
            .generate_login_token(user_uuid)
            .await
    }

    async fn generate_login_token(&mut self, user_uuid: uuid::Uuid) -> Result<String, Status> {
        use rocket_db_pools::diesel::prelude::*;
        use schema::sessions::{self, dsl::*};

        let login_token = cryptography::gen_login_token()
            .map_err(|_| Status::InternalServerError)?;

        let lines_affected = diesel::insert_into(sessions::table)
            .values((
                id.eq(&login_token),
                user_id.eq(user_uuid),
            ))
            .execute(self)
            .await
            .map_err(|_| Status::InternalServerError)?;

        if lines_affected == 1 {
            Ok(login_token)
        } else {
            Err(Status::InternalServerError)
        }
    }

    async fn verify_login_token(&mut self, login_token: &str) -> Result<User, Status> {
        use rocket_db_pools::diesel::prelude::*;
        use schema::sessions::{self};
        use schema::users::{self};

        users::table
            .inner_join(sessions::table.on(users::id.eq(users::id)))
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
