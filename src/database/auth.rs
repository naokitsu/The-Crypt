use diesel::result::Error;
use rocket::serde::{Deserialize, Serialize};
use crate::database::Db;
use crate::models::Token;

pub(crate) trait AuthDatabase {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, ()>;
    async fn register(&mut self, login: &str, password: &str) -> Result<(), ()>;
}


impl AuthDatabase for rocket_db_pools::Connection<Db> {
    async fn login(&mut self, login: &str, password: &str) -> Result<Token, ()> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::{users, secrets};

        let (user_salted_hash, user_id) = users::table
            .inner_join(secrets::table)
            .select((secrets::salted_hash, users::id))
            .filter(users::name.eq(login))
            .first::<(Vec<u8>, uuid::Uuid)>(self)
            .await
            .map_err(|_| ())?;

        if user_salted_hash.len() <= 16 { return Err(()) }
        let (salt, hash) = user_salted_hash
            .split_at(16);

        if verify_password(salt, password.as_bytes(), hash).map_err(|_| ())? {
            generate_token(user_id)
                .map(|access_token| Token {access_token})
                .map_err(|_| ())
        } else {
            Err(())
        }
    }

    async fn register(&mut self, login: &str, password: &str) -> Result<(), ()> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::{users, secrets};

        let salt_hash = hash_password(password.as_bytes()).map_err(|_| ())?;;

        let user_uuid = diesel::insert_into(users::table)
            .values((
                users::name.eq(login),
            ))
            .returning(users::id)
            .get_result::<uuid::Uuid>(self)
            .await
            .map_err(|_| ())?;;

        diesel::insert_into(secrets::table)
            .values((
                secrets::user_id.eq(user_uuid),
                secrets::salted_hash.eq(salt_hash),
            ))
            .execute(self)
            .await
            .map_err(|_| ())?;
        Ok(())
    }
}

fn verify_password(salt: &[u8], password: &[u8], hash: &[u8]) -> Result<bool, openssl::error::ErrorStack> {
    let salt_password = [salt, password].concat();
    let request_hash = openssl::hash::hash(openssl::hash::MessageDigest::sha256(), salt_password.as_slice())?;

    Ok(request_hash.as_ref() == hash)
}

fn hash_password(password: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let mut salt = [0; 16];
    openssl::rand::rand_bytes(&mut salt)?;

    let salt_password = [&salt, password].concat();
    let request_hashed = openssl::hash::hash(openssl::hash::MessageDigest::sha256(), salt_password.as_slice())?;

    let salted_hash = [salt.as_slice(), request_hashed.as_ref()].concat();;

    Ok(salted_hash)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub(crate) fn generate_token(user_id: uuid::Uuid) -> Result<String, ()> {
    use jsonwebtoken::{Algorithm, encode, Header};
    const JWT_SECRET: &[u8] = b"Szechuan Sauce Recipe";

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("") // TODO
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let jwt = encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| ())?;

    Ok(jwt)
}