use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;

use crate::{impl_deserialize_for_vector_wrapper, impl_from_data_json_for, impl_responder_json_for};
use crate::models::member::Member;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Members(Vec<Member>);

impl_deserialize_for_vector_wrapper!(Members, Member);
impl_responder_json_for!(Members);
impl_from_data_json_for!(Members);