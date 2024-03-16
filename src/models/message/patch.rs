use diesel::AsChangeset;
use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Patch<'a> {
    pub content: Option<&'a str>,
}

#[async_trait]
impl<'r> rocket::data::FromData<'r> for Patch<'r> {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        use rocket::serde::json::Json;
        Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
    }
}
