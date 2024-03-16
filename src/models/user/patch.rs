use diesel::AsChangeset;
use rocket::serde::{Deserialize, Serialize};

use crate::impl_from_data_json_for;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Patch<'a> {
    pub name: Option<&'a str>,
}

impl_from_data_json_for!(Patch<'a>);