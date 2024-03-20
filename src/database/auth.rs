use diesel::result::Error;
use rocket::serde::{Deserialize, Serialize};
use crate::database::Db;
use crate::models::{AuthClaims, LoginError, Permissions, RegisterError, Token};

const JWT_SECRET: &[u8] = b"Szechuan Sauce Recipe";


pub(crate) trait AuthDatabase {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, LoginError>;
    async fn register(&mut self, login: &str, password: &str) -> Result<(), RegisterError>;
}


impl AuthDatabase for rocket_db_pools::Connection<Db> {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, LoginError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::{users, secrets};

        let (user_salted_hash, user_id) = users::table
            .inner_join(secrets::table)
            .select((secrets::salted_hash, users::id))
            .filter(users::name.eq(login))
            .first::<(Vec<u8>, uuid::Uuid)>(self)
            .await
            .map_err(|err| match err {
                Error::NotFound => LoginError::Unauthorized,
                _ => LoginError::InternalServerError,
            })?;

        if user_salted_hash.len() <= 16 { return Err(LoginError::InternalServerError) }
        let (salt, hash) = user_salted_hash
            .split_at(16);

        if verify_password(salt, password.as_bytes(), hash).map_err(|_| LoginError::InternalServerError)? {
            generate_token(user_id)
                .map(|access_token| Token {access_token})
                .map_err(|_| LoginError::InternalServerError)
        } else {
            Err(LoginError::Unauthorized)
        }
    }

    async fn register(&mut self, login: &str, password: &str) -> Result<(), RegisterError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::{users, secrets};

        let salt_hash = hash_password(password.as_bytes()).map_err(|_| RegisterError::InternalServerError)?;

        let user_uuid = diesel::insert_into(users::table)
            .values((
                users::name.eq(login),
            ))
            .returning(users::id)
            .get_result::<uuid::Uuid>(self)
            .await
            .map_err(|err| match err {
                Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => RegisterError::Conflict,
                _ => RegisterError::InternalServerError
            })?;;

        diesel::insert_into(secrets::table)
            .values((
                secrets::user_id.eq(user_uuid),
                secrets::salted_hash.eq(salt_hash),
            ))
            .execute(self)
            .await
            .map_err(|_| RegisterError::InternalServerError)?;
        Ok(())
    }
}

pub(crate) fn verify_password(salt: &[u8], password: &[u8], hash: &[u8]) -> Result<bool, openssl::error::ErrorStack> {
    let salt_password = [salt, password].concat();
    let request_hash = openssl::hash::hash(openssl::hash::MessageDigest::sha256(), salt_password.as_slice())?;

    Ok(request_hash.as_ref() == hash)
}

pub(crate) fn hash_password(password: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let mut salt = [0; 16];
    openssl::rand::rand_bytes(&mut salt)?;

    let salt_password = [&salt, password].concat();
    let request_hashed = openssl::hash::hash(openssl::hash::MessageDigest::sha256(), salt_password.as_slice())?;

    let salted_hash = [salt.as_slice(), request_hashed.as_ref()].concat();;

    Ok(salted_hash)
}



pub(crate) fn generate_token(user_id: uuid::Uuid) -> Result<String, ()> {
    use jsonwebtoken::{Algorithm, encode, Header};

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("") // TODO
        .timestamp();

    let claims = AuthClaims {
        sub: user_id.to_string(),
        perms: Permissions::new(
            false,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,

        ),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let jwt = encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| ())?;

    Ok(jwt)
}

pub(crate) fn verify_login_token(token: &str) -> Result<AuthClaims, ()> {
    use jsonwebtoken::{Algorithm, decode, Validation};

    let now = chrono::Utc::now();

    let claims = decode::<AuthClaims>(token, &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET), &Validation::new(Algorithm::HS512))
        .map_err(|_| ()).unwrap();

    if now.timestamp() as usize > claims.claims.exp {
        Err(())
    } else {
        Ok(claims.claims)
    }


}