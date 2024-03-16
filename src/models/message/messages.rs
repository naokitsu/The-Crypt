use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;

use crate::{impl_deserialize_for_vector_wrapper, impl_from_data_json_for, impl_responder_json_for};
use crate::models::message::Message;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Messages<'a>(Vec<Message<'a>>);

impl_deserialize_for_vector_wrapper!(Messages<'a>, Message);
impl_responder_json_for!(Messages<'a>);
impl_from_data_json_for!(Messages<'a>);