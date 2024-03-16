use std::io::Write;

use rocket::{Data, Request};
use rocket::data::{FromData, Outcome};
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_contrib::databases::diesel::{deserialize, not_none, SqlType};
use rocket_contrib::databases::diesel::pg::Pg;
use rocket_db_pools::diesel::{AsChangeset, Insertable, Queryable};
use rocket_db_pools::diesel::deserialize::{FromSql, FromSqlRow};
use rocket_db_pools::diesel::serialize::{self, IsNull, Output, ToSql};

use crate::schema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Member {
    pub user_id: uuid::Uuid,
    pub channel_id: uuid::Uuid,
    pub role: UserRole,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::user_channel)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Patch {
    pub role: Option<UserRole>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::user_channel)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Insert {
    pub user_id: uuid::Uuid,
    pub role: UserRole,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum UserRole {
    Admin,
    Member,
}

impl FromSql<schema::sql_types::UserRole, Pg> for UserRole {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"admin" => Ok(UserRole::Admin),
            b"member" => Ok(UserRole::Member),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<schema::sql_types::UserRole, Pg> for UserRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            UserRole::Admin => out.write_all(b"foo")?,
            UserRole::Member => out.write_all(b"bar")?,
        }
        Ok(IsNull::No)
    }
}

/*
impl diesel::Queryable<schema::sql_types::UserRole, diesel::pg::Pg> for UserRole {
    type Row = Self;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(row)
    }
}

impl diesel::deserialize::FromSql<schema::sql_types::UserRole, diesel::pg::Pg> for UserRole {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"admin" => Ok(UserRole::Admin),
            _ => Ok(UserRole::Member),
        }
    }
}*/

/*impl diesel::query_builder::QueryFragment<diesel::pg::Pg> for UserRole {
    fn walk_ast<'b>(&'b self, mut out: diesel::query_builder::AstPass<'_, 'b, diesel::pg::Pg>) -> diesel::QueryResult<()> {
        out.push_sql(match self {
            UserRole::Admin => "admin",
            UserRole::Member => "member",
        });
        Ok(())
    }
}*/

impl diesel::query_builder::QueryId for UserRole {
    type QueryId = uuid::Uuid;
    const HAS_STATIC_QUERY_ID: bool = true;
}

impl diesel::Expression for UserRole {
    type SqlType = schema::sql_types::UserRole;
}

impl<T> diesel::AppearsOnTable<T> for UserRole {

}

#[async_trait]
impl<'r> Responder<'r, 'static> for Member {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(request)
    }
}

#[async_trait]
impl<'r> FromData<'r> for Patch {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
    }
}


#[async_trait]
impl<'r> FromData<'r> for Insert {
    type Error = rocket::serde::json::Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
    }
}
