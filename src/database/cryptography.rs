use chrono::Utc;
use jsonwebtoken::{Algorithm, encode, Header};
use openssl::error::ErrorStack;
use openssl::hash::hash as hash_openssl;
use openssl::rand::rand_bytes;
use rocket::serde::{Deserialize, Serialize};

const MESSAGE_DIGEST: fn() -> openssl::hash::MessageDigest = openssl::hash::MessageDigest::sha256;
//const MESSAGE_DIGEST_SIZE: usize = 32;
//const TOKEN_SIZE_U8: usize = 54;
//const TOKEN_SIZE_BASE64: usize = TOKEN_SIZE_U8 * 4 / 3;
pub(super) const SALT_SIZE: usize = 16;
//const HASH_SIZE: usize = 32;
//const SALTED_SIZE: usize = SALT_SIZE + HASH_SIZE;

const JWT_SECRET: &[u8] = b"Szechuan Sauce Recipe";


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub(super) enum TokenGenError {
    JWTTokenError
}

pub(super) enum TokenVerifyError {
    Expired,
    JWTTokenError,
}

pub(crate) fn gen_login_token(user_id: uuid::Uuid, is_admin: bool) -> Result<String, TokenGenError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60*60))
        .expect("") // TODO
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        role: if is_admin { "admin" } else { "user" }.to_string(),
        exp: expiration as usize,
    };
    let header = Header {
        alg: Algorithm::HS256,
        typ: Some("JWT".to_string()),
        ..Default::default()
    };

    let jwt = encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| TokenGenError::JWTTokenError)?;

    Ok(jwt)
}

pub(crate) fn verify_login_token(token: &[u8; 32]) -> Result<(), TokenVerifyError> {
    Ok(())
}

pub(super) fn verify_password(salt: &[u8], password: &[u8], hash: &[u8]) -> Result<bool, ErrorStack> {
    let salt_password = [salt, password].concat();
    let request_hash = hash_openssl(MESSAGE_DIGEST(), salt_password.as_slice())?;

    Ok(request_hash.as_ref() == hash)
}

pub(super) fn hash_password(password: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let mut salt = [0; SALT_SIZE];
    rand_bytes(&mut salt)?;

    let salt_password = [&salt, password].concat();
    let request_hashed = hash_openssl(MESSAGE_DIGEST(), salt_password.as_slice())?;
    let salted_hash = [salt.as_slice(), request_hashed.as_ref()].concat();

    Ok(salted_hash)
}