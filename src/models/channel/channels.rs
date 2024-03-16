use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;

use crate::{impl_deserialize_for_vector_wrapper, impl_from_data_json_for, impl_responder_json_for};
use crate::models::channel::Channel;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Channels<'a>(Vec<Channel<'a>>);

impl_deserialize_for_vector_wrapper!(Channels<'a>, Channel);
impl_responder_json_for!(Channels<'a>);
impl_from_data_json_for!(Channels<'a>);