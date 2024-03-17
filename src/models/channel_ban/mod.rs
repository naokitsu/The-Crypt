use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::Queryable;

use crate::{impl_from_data_json_for, impl_responder_json_for};
use crate::models::Model;

pub(crate) mod channel_bans;

impl Model for ChannelBan {
    type Patch = ();
    type Insert = Self;
    type Vector = channel_bans::ChannelBans;

    fn to_patch(&self) -> Self::Patch {
        ()
    }

    fn to_insert(&self) -> Self::Insert {
        self.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::bans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct ChannelBan {
    user_id: uuid::Uuid,
    channel_id: uuid::Uuid,
}

impl_responder_json_for!(ChannelBan);
impl_from_data_json_for!(ChannelBan);