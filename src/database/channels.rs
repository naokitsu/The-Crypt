use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::query_builder::NoFromClause;
use diesel::result::Error;
use rocket_contrib::databases::diesel::result::Error::DeserializationError;
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::diesel::{RunQueryDsl, QueryDsl};
use crate::{models, schema};
use crate::schema::channels;


pub(crate) enum DataRetrievalError {
    NotFound,
    InternalError,
}

pub(crate) enum DataInsertionError {
    AlreadyExists,
    InternalError,
}

pub(crate) enum DataRemovalError {
    InternalError,
}

pub(crate) enum DataSetError {
    InternalError,
}

pub trait Database {
    type Id<'a>;
    type UserID<'a>;
    type Channel;
    type Patch;
    type Insert;

    async fn get_channel(&mut self, channel_id: Self::Id<'_>) -> Result<Self::Channel, DataRetrievalError>;

    async fn insert_channel(
        &mut self,
        channel: Self::Insert,
    ) -> Result<Self::Channel, DataInsertionError>;

    async fn patch_channel(&mut self, channel_id: Self::Id<'_>, patch: Self::Patch) -> Result<Self::Channel, DataSetError>;

    async fn remove_session(&mut self, user: Self::Id<'_>) -> Result<Self::Channel, DataRemovalError>;

    async fn get_user_role_in_channel(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>) -> Result<UserRole, DataRetrievalError>;

    async fn insert_user_channel_relation(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>, role: UserRole) -> Result<(), DataInsertionError>;
}

impl Database for rocket_db_pools::Connection<crate::database::Db> {
    type Id<'a> = uuid::Uuid;
    type UserID<'a> = uuid::Uuid;

    type Channel = models::Channel;
    type Patch = models::ChannelPatch;
    type Insert = models::ChannelInsert;

    async fn get_channel(&mut self, channel_id: Self::Id<'_>) -> Result<Self::Channel, DataRetrievalError> {
        channels::table
            .filter(channels::id.eq(channel_id))
            .get_result(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataRetrievalError::NotFound,
                _ => DataRetrievalError::InternalError,
            })
    }

    async fn insert_channel(&mut self, channel: Self::Insert) -> Result<Self::Channel, DataInsertionError> {
        diesel::insert_into(channels::table)
            .values(channel)
            .returning(channels::all_columns)
            .get_result(self)
            .await
            .map_err(|e| DataInsertionError::InternalError)
    }


    async fn patch_channel(&mut self, channel_id: Self::Id<'_>, mut patch: Self::Patch) -> Result<Self::Channel, DataSetError> {
        patch.id = None;
        diesel::update(channels::table)
            .set(patch)
            .filter(channels::id.eq(channel_id))
            .returning(channels::all_columns)
            .get_result(self)
            .await
            .map_err(|e| match e {
                Error::DatabaseError(_, _) => DataSetError::InternalError,
                _ => DataSetError::InternalError,
            })
    }

    async fn remove_session(&mut self, channel_id: Self::Id<'_>) -> Result<Self::Channel, DataRemovalError> {
        diesel::delete(channels::table)
            .filter(channels::id.eq(channel_id))
            .returning(channels::all_columns)
            .get_result(self)
            .await
            .map_err(|e| match e {
                _ => DataRemovalError::InternalError,
            })
    }

    async fn get_user_role_in_channel(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>) -> Result<UserRole, DataRetrievalError> {
        schema::user_channel::table
            .select(schema::user_channel::role)
            .filter(schema::user_channel::user_id.eq(user_id))
            .filter(schema::user_channel::channel_id.eq(channel_id))
            .get_result(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataRetrievalError::NotFound,
                _ => DataRetrievalError::InternalError,
            })
    }

    async fn insert_user_channel_relation(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>, role: UserRole) -> Result<(), DataInsertionError> {
        let _ = diesel::insert_into(schema::user_channel::table)
            .values((schema::user_channel::user_id.eq(user_id), schema::user_channel::channel_id.eq(channel_id), schema::user_channel::role.eq(role)))
            .execute(self)
            .await
            .map_err(|e| match e {
                _ => DataInsertionError::InternalError,
            });
        todo!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    User,
}

// UserRole is an enum "admin" or "user"

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