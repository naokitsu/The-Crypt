use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::Queryable;

use crate::{impl_from_data_json_for, impl_responder_json_for};
use crate::models::member::role::MemberRole;
use crate::models::Model;

pub(crate) mod members;
pub(crate) mod patch;
pub(crate) mod insert;
pub(crate) mod role;

impl Model for Member {
    type Patch = patch::Patch;
    type Insert = insert::Insert;
    type Vector = members::Members;

    fn to_patch(&self) -> Self::Patch {
        Self::Patch {
            role: Some(self.role)
        }
    }

    fn to_insert(&self) -> Self::Insert {
        Self::Insert {
            role: Some(self.role)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Member {
    pub user_id: uuid::Uuid,
    pub channel_id: uuid::Uuid,
    pub role: MemberRole,
}

impl_responder_json_for!(Member);
impl_from_data_json_for!(Member);