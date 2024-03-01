use std::io::{Read, Write};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use openssl::error::ErrorStack;
use openssl::rand::rand_bytes;
use crate::database::{LoginError, SALT_SIZE};
fn get_message_digest() -> openssl::hash::MessageDigest {
    openssl::hash::MessageDigest::sha256()
}

pub(crate) fn gen_login_token() -> Result<String, ErrorStack> {
    let mut buf = [0; 54];
    rand_bytes(&mut buf)?;
    let token = STANDARD.encode(buf.as_ref());
    Ok(token)
}

pub(super) fn verify_password(salt: &[u8], password: &[u8], hash: &[u8]) -> Result<bool, ErrorStack> {
    let salt_password = [salt, password].concat();
    let request_hash = openssl::hash::hash(get_message_digest(), salt_password.as_slice())?;

    Ok(request_hash.as_ref() == hash)
}

pub(super) fn hash_password(password: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let mut salt = [0u8; SALT_SIZE];
    rand_bytes(&mut salt)?;

    let salt_password = [&salt, password].concat();
    let request_hashed = openssl::hash::hash(get_message_digest(), salt_password.as_slice())?;

    let mut hashed_slice = [0u8; 32];
    hashed_slice.copy_from_slice(&request_hashed);
    let salted_hash = [salt.as_slice(), hashed_slice.as_slice()].concat();;

    Ok(salted_hash)
}