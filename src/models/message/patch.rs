use diesel::AsChangeset;
use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};

use crate::impl_from_data_json_for;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Patch<'a> {
    pub content: Option<&'a str>,
}

impl_from_data_json_for!(Patch<'a>);