use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;
use crate::impl_from_data_json_for;

use crate::models::user::User;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Users<'a>(Vec<User<'a>>);

impl<'de> Deserialize<'de> for Users<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(Users(Vec::<User>::deserialize(deserializer)?))
    }
}

impl_from_data_json_for!(Users<'a>);