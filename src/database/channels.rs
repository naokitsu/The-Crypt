use diesel::result::Error;
use rocket_db_pools::diesel::prelude::*;
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
    type Member;
    type Channel;
    type ChannelPatch;
    type ChannelInsert;
    type MemberPatch;
    type MemberInsert;
    type Message;

    async fn get_channel(&mut self, channel_id: Self::Id<'_>) -> Result<Self::Channel, DataRetrievalError>;

    async fn insert_channel(
        &mut self,
        channel: Self::ChannelInsert,
    ) -> Result<Self::Channel, DataInsertionError>;

    async fn patch_channel(&mut self, channel_id: Self::Id<'_>, patch: Self::ChannelPatch) -> Result<Self::Channel, DataSetError>;

    async fn remove_channel(&mut self, user: Self::Id<'_>) -> Result<Self::Channel, DataRemovalError>;

    async fn get_members(&mut self, channel_id: Self::Id<'_>) -> Result<Vec<Self::Member>, DataRetrievalError>;

    async fn get_member(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>) -> Result<Self::Member, DataRetrievalError>;

    async fn insert_member(&mut self, channel_id: Self::Id<'_>, member: Self::MemberInsert) -> Result<Self::Member, DataInsertionError>;

    async fn patch_member(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>, member: Self::MemberPatch) -> Result<Self::Member, DataSetError>;
    async fn remove_member(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>) -> Result<Self::Member, DataRemovalError>;

    async fn get_messages(&mut self, channel_id: Self::Id<'_>) -> Result<Vec<Self::Message>, DataRetrievalError>;
    async fn get_message(&mut self, channels_id: Self::Id<'_>, message_id: Self::Id<'_>) -> Result<Self::Message, DataRetrievalError>;

    async fn insert_message(&mut self, message: Self::Message) -> Result<Self::Message, DataInsertionError>;

    async fn remove_message(&mut self, message_id: Self::Id<'_>) -> Result<Self::Message, DataRemovalError>;
}

impl Database for rocket_db_pools::Connection<crate::database::Db> {
    type Id<'a> = uuid::Uuid;
    type UserID<'a> = uuid::Uuid;
    type Member = models::Member;

    type Channel = models::Channel;
    type ChannelPatch = models::ChannelPatch;
    type ChannelInsert = models::ChannelInsert;

    type MemberPatch = models::MemberPatch;

    type MemberInsert = models::MemberInsert;

    type Message = models::Message;

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

    async fn insert_channel(&mut self, channel: Self::ChannelInsert) -> Result<Self::Channel, DataInsertionError> {
        diesel::insert_into(channels::table)
            .values(channel)
            .returning(channels::all_columns)
            .get_result(self)
            .await
            .map_err(|_| DataInsertionError::InternalError)
    }


    async fn patch_channel(&mut self, channel_id: Self::Id<'_>, mut patch: Self::ChannelPatch) -> Result<Self::Channel, DataSetError> {
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

    async fn remove_channel(&mut self, channel_id: Self::Id<'_>) -> Result<Self::Channel, DataRemovalError> {
        diesel::delete(channels::table)
            .filter(channels::id.eq(channel_id))
            .returning(channels::all_columns)
            .get_result(self)
            .await
            .map_err(|e| match e {
                _ => DataRemovalError::InternalError,
            })
    }

    async fn get_members(&mut self, channel_id: Self::Id<'_>) -> Result<Vec<Self::Member>, DataRetrievalError> {
        schema::user_channel::table
            .filter(schema::user_channel::channel_id.eq(channel_id))
            .get_results(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataRetrievalError::NotFound,
                _ => DataRetrievalError::InternalError,
            })
    }

    async fn get_member(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>) -> Result<Self::Member, DataRetrievalError> {
        schema::user_channel::table
            .filter(schema::user_channel::user_id.eq(user_id))
            .filter(schema::user_channel::channel_id.eq(channel_id))
            .get_result(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataRetrievalError::NotFound,
                _ => DataRetrievalError::InternalError,
            })
    }

    async fn insert_member(&mut self, channel_id: Self::Id<'_>, member: Self::MemberInsert) -> Result<Self::Member, DataInsertionError> {
        diesel::insert_into(schema::user_channel::table)
            .values(member)
            .returning(schema::user_channel::all_columns)
            .get_result(self)
            .await
            .map_err(|e| {
                println!("{:?}", e);
                DataInsertionError::InternalError
            })
    }

    async fn patch_member(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>, member: Self::MemberPatch) -> Result<Self::Member, DataSetError> {
        diesel::update(schema::user_channel::table)
            .set(member)
            .filter(schema::user_channel::channel_id.eq(channel_id))
            .returning(schema::user_channel::all_columns)
            .get_result(self)
            .await
            .map_err(|e| match e {
                Error::DatabaseError(_, _) => DataSetError::InternalError,
                _ => DataSetError::InternalError,
            })
    }

    async fn remove_member(&mut self, channel_id: Self::Id<'_>, user_id: Self::UserID<'_>) -> Result<Self::Member, DataRemovalError> {
        diesel::delete(schema::user_channel::table)
            .filter(schema::user_channel::channel_id.eq(channel_id))
            .filter(schema::user_channel::user_id.eq(user_id))
            .returning(schema::user_channel::all_columns)
            .get_result(self)
            .await
            .map_err(|e| match e {
                _ => DataRemovalError::InternalError,
            })
    }

    async fn get_messages(&mut self, channel_id: Self::Id<'_>) -> Result<Vec<Self::Message>, DataRetrievalError> {
        schema::messages::table
            .filter(schema::messages::channel_id.eq(channel_id))
            .limit(50)
            .get_results(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataRetrievalError::NotFound,
                _ => DataRetrievalError::InternalError,
            })
    }

    async fn get_message(&mut self, channels_id: Self::Id<'_>, message_id: Self::Id<'_>) -> Result<Self::Message, DataRetrievalError> {
        schema::messages::table
            .filter(schema::messages::id.eq(message_id))
            .filter(schema::messages::channel_id.eq(channels_id))
            .get_result(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataRetrievalError::NotFound,
                _ => DataRetrievalError::InternalError,
            })
    }

    async fn insert_message(&mut self, message: Self::Message) -> Result<Self::Message, DataInsertionError> {
        diesel::insert_into(schema::messages::table)
            .values(message)
            .returning(schema::messages::all_columns)
            .get_result(self)
            .await
            .map_err(|e| DataInsertionError::InternalError)
    }

    async fn remove_message(&mut self, message_id: Self::Id<'_>) -> Result<Self::Message, DataRemovalError> {
        diesel::delete(schema::messages::table)
            .filter(schema::messages::id.eq(message_id))
            .returning(schema::messages::all_columns)
            .get_result(self)
            .await
            .map_err(|e| match e {
                _ => DataRemovalError::InternalError,
            })
    }
}
