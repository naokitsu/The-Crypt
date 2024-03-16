use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::Queryable;

use crate::{impl_from_data_json_for, impl_responder_json_for};
use crate::models::Model;

mod users;
mod patch;
mod insert;

impl<'a> Model for User<'a> {
    type Patch = patch::Patch<'a>;
    type Insert = insert::Insert<'a>;
    type Vector = users::Users<'a>;

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
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
struct User<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub is_admin: bool,
}

impl_responder_json_for!(User<'a>);
impl_from_data_json_for!(User<'a>);

