use std::str::FromStr;
use rocket::request::FromParam;

// uuid::Uuid wrapper because i can't implement traits for the uuid::Uuid
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UUIDWrapper(pub uuid::Uuid);

impl<'a> FromParam<'a> for UUIDWrapper {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        let id = <uuid::Uuid as FromStr>::from_str(param).map_err(|_| param)?;
        Ok(UUIDWrapper(id))
    }
}

impl From<UUIDWrapper> for uuid::Uuid {
    fn from(id: UUIDWrapper) -> Self {
        id.0
    }
}