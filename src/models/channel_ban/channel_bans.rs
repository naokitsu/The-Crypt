use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;

use crate::{impl_deserialize_for_vector_wrapper, impl_from_data_json_for, impl_responder_json_for};
use crate::models::channel_ban::ChannelBan;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ChannelBans(Vec<ChannelBan>);

impl_deserialize_for_vector_wrapper!(ChannelBans, ChannelBan);
impl_responder_json_for!(ChannelBans);
impl_from_data_json_for!(ChannelBans);