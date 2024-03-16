use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;
use crate::impl_from_data_json_for;

use crate::models::channel::Channel;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Channels<'a>(Vec<Channel<'a>>);

impl<'de> Deserialize<'de> for Channels<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(Channels(Vec::<Channel>::deserialize(deserializer)?))
    }
}

impl_from_data_json_for!(Channels<'a>);