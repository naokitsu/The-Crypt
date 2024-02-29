use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use crate::auth::objects::Error;

use rocket::serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[post("/login", format = "json", data = "<login_request>")]
pub(super) fn login(login_request: Option<Json<LoginRequest>>) -> Result<String, Error> {
    if let Some(x) = login_request {
        Ok(format!("{}:{}", x.username, x.password))
    } else {
        Err(Error::BadRequest("Missing username or password".to_string()))
    }
}

#[post("/register", format = "json", data = "<login_request>")]
pub(super) fn register(login_request: Option<Json<LoginRequest>>) -> Result<String, Error> {
    if let Some(x) = login_request {
        Ok(format!("{}:{}", x.username, x.password))
    } else {
        Err(Error::BadRequest("Missing username or password".to_string()))
    }
}