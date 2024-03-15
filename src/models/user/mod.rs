use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket_db_pools::diesel::{AsChangeset, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

use crate::models::Model;
use crate::{use_json_responder, use_json_responder_with_clone};

mod users;
mod patch;
mod insert;

impl<'a> Model for User<'a> {
    type Patch = patch::Patch<'a>;
    type Insert = insert::Insert<'a>;
    type Vector = users::Users<'a>;

    fn to_patch(&self) -> Self::Patch {
        Self::Patch {
            id: self.id,
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
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
struct User<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub is_admin: bool,
}

// --- Json Responder ---

#[async_trait]
impl<'r> rocket::response::Responder<'r, 'r> for User<'_> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'r> {
        rocket::serde::json::Json(self).respond_to(request)
    }
}

impl<'r> rocket::data::FromData<'r> for User<'_> {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        use rocket::serde::json::Json;
        Json::from_data(req, data).await.map(|json: Json<User>| json.into_inner())
    }
}


