use diesel::pg::PgValue;
use diesel::query_builder::NoFromClause;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};
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


#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum UserRole {
    Admin,
    User,
}

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
            _ => Ok(UserRole::User),
        }
    }
}

impl diesel::query_builder::QueryFragment<diesel::pg::Pg> for UserRole {
    fn walk_ast<'b>(&'b self, mut out: diesel::query_builder::AstPass<'_, 'b, diesel::pg::Pg>) -> diesel::QueryResult<()> {
        out.push_sql(match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        });
        Ok(())
    }
}

impl diesel::query_builder::QueryId for UserRole {
    type QueryId = uuid::Uuid;
    const HAS_STATIC_QUERY_ID: bool = true;
}

impl diesel::Expression for UserRole {
    type SqlType = schema::sql_types::UserRole;
}

impl diesel::AppearsOnTable<NoFromClause> for UserRole {

}