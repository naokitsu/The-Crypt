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