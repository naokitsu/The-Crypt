use rocket::response::Responder;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::impl_responder_json_for;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub access_token: String,
}

impl_responder_json_for!(Token);

impl<'a> Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Token", 2)?;
        state.serialize_field("access_token", &self.access_token)?;
        state.serialize_field("token_type", "BEARER")?;
        state.end()
    }
}