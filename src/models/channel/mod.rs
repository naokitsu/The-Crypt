use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::Queryable;

use crate::models::Model;

mod channels;
mod patch;
mod insert;

impl<'a> Model for Channel<'a> {
    type Patch = patch::Patch<'a>;
    type Insert = insert::Insert<'a>;
    type Vector = channels::Channels<'a>;

    fn to_patch(&self) -> Self::Patch {
        Self::Patch {
            name: Some(self.name)
        }
    }

    fn to_insert(&self) -> Self::Insert {
        Self::Insert {
            name: self.name
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
struct Channel<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
}

#[async_trait]
impl<'r> rocket::response::Responder<'r, 'r> for Channel<'_> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'r> {
        rocket::serde::json::Json(self).respond_to(request)
    }
}

#[async_trait]
impl<'r> rocket::data::FromData<'r> for Channel<'r> {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        use rocket::serde::json::Json;
        Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
    }
}

