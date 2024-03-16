use diesel::Insertable;
use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};

use crate::impl_from_data_json_for;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Insert<'a> {
    pub name: &'a str,
}

impl_from_data_json_for!(Insert<'a>);