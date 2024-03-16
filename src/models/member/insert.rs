use diesel::Insertable;
use rocket::serde::{Deserialize, Serialize};

use crate::impl_from_data_json_for;
use crate::models::member::role::MemberRole;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Insert {
    pub role: Option<MemberRole>,
}

impl_from_data_json_for!(Insert);