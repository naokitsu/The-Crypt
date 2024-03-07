use std::str::FromStr;
use rocket::request::FromParam;

// uuid::Uuid wrapper because i can't implement traits for the uuid::Uuid
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChannelId(pub uuid::Uuid);

impl<'a> FromParam<'a> for ChannelId {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        let id = <uuid::Uuid as FromStr>::from_str(param).map_err(|_| param)?;
        Ok(ChannelId(id))
    }
}

impl From<ChannelId> for uuid::Uuid {
    fn from(id: ChannelId) -> Self {
        id.0
    }
}