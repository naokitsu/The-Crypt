use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::Queryable;
use crate::impl_from_data_json_for;

use crate::models::Model;

mod messages;
mod patch;
mod insert;

impl<'a> Model for Message<'a> {
    type Patch = patch::Patch<'a>;
    type Insert = insert::Insert<'a>;
    type Vector = messages::Messages<'a>;

    fn to_patch(&self) -> Self::Patch {
        Self::Patch {
            content: Some(self.content)
        }
    }

    fn to_insert(&self) -> Self::Insert {
        Self::Insert {
            user_id: self.user_id,
            channel_id: self.channel_id,
            content: self.content
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
struct Message<'a> {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub channel_id: uuid::Uuid,
    pub content: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[async_trait]
impl<'r> rocket::response::Responder<'r, 'r> for Message<'_> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'r> {
        rocket::serde::json::Json(self).respond_to(request)
    }
}

impl_from_data_json_for!(Message<'a>);