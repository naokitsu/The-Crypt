use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;

use crate::models::message::Message;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Messages<'a>(Vec<Message<'a>>);

impl<'de> Deserialize<'de> for Messages<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(Messages(Vec::<Message>::deserialize(deserializer)?))
    }
}

#[async_trait]
impl<'r> rocket::data::FromData<'r> for Messages<'r> {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        use rocket::serde::json::Json;
        Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
    }
}
