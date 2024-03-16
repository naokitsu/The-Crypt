use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::Queryable;
use crate::impl_from_data_json_for;

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

impl_from_data_json_for!(Channel<'a>);