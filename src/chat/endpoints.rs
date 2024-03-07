use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::database::{Db, AuthDatabase};
use crate::models;
use crate::models::{Channel, ChannelError, ChannelId};
use crate::schema::messages::channel_id;
use crate::database::channels::{Database, DataInsertionError, DataRemovalError, DataRetrievalError, DataSetError};

#[get("/channels/<id>")]
pub async fn get_channel_by_id(id: models::ChannelId, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    db.get_channel(id.into())
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
}

#[patch("/channels/<id>", format = "json", data = "<patch>")]
pub async fn patch_channel_by_id(id: models::ChannelId, patch: models::ChannelPatch, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    db.patch_channel(id.into(), patch)
        .await
        .map_err(|e| match e {
            DataSetError::InternalError => ChannelError::InternalServerError,
        })
}

#[delete("/channels/<id>")]
pub async fn remove_channel_by_id(id: models::ChannelId, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    db.remove_session(id.into())
        .await
        .map_err(|e| match e {
            DataRemovalError::InternalError => ChannelError::InternalServerError,
        })
}

#[post("/channels", format = "json", data = "<channel>")]
pub async fn create_channel(channel: models::ChannelInsert, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    db.insert_channel(channel)
        .await
        .map_err(|e| match e {
            DataInsertionError::AlreadyExists => ChannelError::Conflict,
            DataInsertionError::InternalError => ChannelError::InternalServerError,
        })
}