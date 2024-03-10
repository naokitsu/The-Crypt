use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::database::{Db, AuthDatabase};
use crate::models;
use crate::models::{Channel, ChannelError, ChannelId, Member, User, UserRole};
use crate::schema::messages::channel_id;
use crate::database::channels::{Database, DataInsertionError, DataRemovalError, DataRetrievalError, DataSetError};

#[get("/channels/<id>")]
pub async fn get_channel_by_id(id: models::ChannelId, user: User, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    let _ = db.get_user_role_in_channel(id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })?;

    db.get_channel(id.into())
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
}

#[patch("/channels/<id>", format = "json", data = "<patch>")]
pub async fn patch_channel_by_id(id: models::ChannelId, user: User, patch: models::ChannelPatch, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    db.get_user_role_in_channel(id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
        .and_then(|role| if role == UserRole::Admin { Ok(()) } else { Err(ChannelError::Unauthorized) })?;

    db.patch_channel(id.into(), patch)
        .await
        .map_err(|e| match e {
            DataSetError::InternalError => ChannelError::InternalServerError,
        })
}

#[delete("/channels/<id>")]
pub async fn remove_channel_by_id(id: models::ChannelId, user: User, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    db.get_user_role_in_channel(id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
        .and_then(|role| if role == UserRole::Admin { Ok(()) } else { Err(ChannelError::Unauthorized) })?;

    db.remove_session(id.into())
        .await
        .map_err(|e| match e {
            DataRemovalError::InternalError => ChannelError::InternalServerError,
        })
}

#[post("/channels", format = "json", data = "<channel>")]
pub async fn create_channel(mut channel: models::ChannelInsert, user: User, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    let new_channel = db.insert_channel(channel)
        .await
        .map_err(|e| match e {
            DataInsertionError::AlreadyExists => ChannelError::Conflict,
            DataInsertionError::InternalError => ChannelError::InternalServerError,
        })?;

    db.insert_user_channel_relation(new_channel.id, user.id, UserRole::Admin)
        .await
        .map_err(|e| match e {
            _ => ChannelError::InternalServerError,
        })?;
    Ok(new_channel)
}

#[get("/channels/<id>/members")]
pub async fn get_channel_members(id: models::ChannelId, mut db: Connection<Db>) -> Result<Json<Vec<Member>>, ChannelError> {
    db.get_channel_relations(id.into())
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
        .map(Json)
}