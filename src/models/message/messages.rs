use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;
use crate::impl_from_data_json_for;

use crate::models::message::Message;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Messages<'a>(Vec<Message<'a>>);

impl<'de> Deserialize<'de> for Messages<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(Messages(Vec::<Message>::deserialize(deserializer)?))
    }
}

impl_from_data_json_for!(Messages<'a>);