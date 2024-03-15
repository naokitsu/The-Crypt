use rocket::Request;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::ser::SerializeStruct;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub access_token: String,
}

#[async_trait]
impl<'r> Responder<'r, 'static> for Token {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(_request)
    }
}

impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Token", 2)?;
        state.serialize_field("access_token", &self.access_token)?;
        state.serialize_field("token_type", "BEARER")?;
        state.end()
    }
}