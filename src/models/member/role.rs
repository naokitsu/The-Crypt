use diesel_derive_enum::DbEnum;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::MemberRole"]
pub enum MemberRole {
    Owner,
    Admin,
    Member,
}