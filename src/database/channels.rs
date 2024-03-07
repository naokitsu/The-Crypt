use diesel::expression::AsExpression;
use diesel::result::Error;
use crate::schema::channels;
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::diesel::{RunQueryDsl, QueryDsl};
use crate::{models, schema};


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


    async fn insert_channel(&mut self, channel: Self::Insert) -> Result<Self::Channel, DataInsertionError> {
        diesel::insert_into(channels::table)
            .values(channel)
            .returning(channels::all_columns)
            .get_result(self)
            .await
            .map_err(|e| DataInsertionError::InternalError)
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
}