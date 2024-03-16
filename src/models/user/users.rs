use rocket::serde::{Deserialize, Serialize};
use serde::Deserializer;

use crate::{impl_deserialize_for_vector_wrapper, impl_from_data_json_for, impl_responder_json_for};
use crate::models::user::User;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Users<'a>(Vec<User<'a>>);


impl_deserialize_for_vector_wrapper!(Users<'a>, User);
impl_responder_json_for!(Users<'a>);
impl_from_data_json_for!(Users<'a>);
