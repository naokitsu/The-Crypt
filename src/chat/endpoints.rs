use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::database::channels::{Database, DataInsertionError, DataRemovalError, DataRetrievalError, DataSetError};
use crate::database::Db;
use crate::models;
use crate::models::{Channel, ChannelError, Member, MemberInsert, MemberPatch, Message, User, UserRole};

#[get("/channels/<id>")]
pub async fn get_channel_by_id(id: models::UUIDWrapper, user: User, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    let _ = db.get_member(id.into(), user.id)
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
pub async fn patch_channel_by_id(id: models::UUIDWrapper, user: User, patch: models::ChannelPatch, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    db.get_member(id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
        .and_then(|member| if member.role == UserRole::Admin { Ok(()) } else { Err(ChannelError::Unauthorized) })?;

    db.patch_channel(id.into(), patch)
        .await
        .map_err(|e| match e {
            DataSetError::InternalError => ChannelError::InternalServerError,
        })
}

#[delete("/channels/<id>")]
pub async fn remove_channel_by_id(id: models::UUIDWrapper, user: User, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    db.get_member(id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
        .and_then(|member| if member.role == UserRole::Admin { Ok(()) } else { Err(ChannelError::Unauthorized) })?;

    db.remove_channel(id.into())
        .await
        .map_err(|e| match e {
            DataRemovalError::InternalError => ChannelError::InternalServerError,
        })
}

#[post("/channels", format = "json", data = "<channel>")]
pub async fn create_channel(channel: models::ChannelInsert, user: User, mut db: Connection<Db>) -> Result<Channel, ChannelError> {
    let new_channel = db.insert_channel(channel)
        .await
        .map_err(|e| match e {
            DataInsertionError::AlreadyExists => ChannelError::Conflict,
            DataInsertionError::InternalError => ChannelError::InternalServerError,
        })?;

    println!("Hello");
    db.insert_member(new_channel.id, MemberInsert { user_id: user.id, role: UserRole::Admin })
        .await
        .map_err(|e| match e {
            _ => ChannelError::InternalServerError,
        })?;
    Ok(new_channel)
}

#[get("/channels/<id>/members")]
pub async fn get_channel_members(id: models::UUIDWrapper, user: User, mut db: Connection<Db>) -> Result<Json<Vec<Member>>, ChannelError> {
    db.get_member(id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })?;

    db.get_members(id.into())
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
        .map(Json)
}

#[get("/channels/<channel_id>/members/<user_id>")]
pub async fn get_channel_member(channel_id: models::UUIDWrapper, user_id: models::UUIDWrapper, user: User, mut db: Connection<Db>) -> Result<Member, ChannelError> {
    db.get_member(channel_id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })?;

    db.get_member(channel_id.into(), user_id.into())
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
}

#[post("/channels/<channel_id>/members", format = "json", data = "<member>")]
pub async fn add_channel_member(channel_id: models::UUIDWrapper, member: MemberInsert, user: User, mut db: Connection<Db>) -> Result<Member, ChannelError> {
    let myself = db.get_member(channel_id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })?;

    if myself.role != UserRole::Admin && member.role == UserRole::Admin {
        return Err(ChannelError::Unauthorized);
    }

    db.insert_member(channel_id.into(), member)
        .await
        .map_err(|e| match e {
            DataInsertionError::AlreadyExists => ChannelError::Conflict,
            DataInsertionError::InternalError => ChannelError::InternalServerError,
        })
}

#[patch("/channels/<channel_id>/members/<user_id>", format = "json", data = "<member>")]
pub async fn update_channel_member(channel_id: models::UUIDWrapper, user_id: models::UUIDWrapper, member: MemberPatch, user: User, mut db: Connection<Db>) -> Result<Member, ChannelError> {
    let myself = db.get_member(channel_id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })?;

    if myself.role != UserRole::Admin && member.role == Some(UserRole::Admin) {
        return Err(ChannelError::Unauthorized);
    }
    db.patch_member(channel_id.into(), user_id.into(), member)
        .await
        .map_err(|e| match e {
            DataSetError::InternalError => ChannelError::InternalServerError,
        })
}

#[delete("/channels/<channel_id>/members/<user_id>")]
pub async fn remove_channel_member(channel_id: models::UUIDWrapper, user_id: models::UUIDWrapper, user: User, mut db: Connection<Db>) -> Result<Member, ChannelError> {
    db.get_member(channel_id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
        .and_then(|member| if member.role == UserRole::Admin { Ok(()) } else { Err(ChannelError::Unauthorized) })?;

    db.remove_member(channel_id.into(), user_id.into())
        .await
        .map_err(|e| match e {
            DataRemovalError::InternalError => ChannelError::InternalServerError,
        })
}


#[get("/channels/<id>/messages")]
pub async fn get_channel_messages(id: models::UUIDWrapper, user: User, mut db: Connection<Db>) -> Result<Json<Vec<Message>>, ChannelError> {
    db.get_member(id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })?;

    db.get_messages(id.into())
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
        .map(Json)
}

#[get("/channels/<channel_id>/messages/<message_id>")]
pub async fn get_channel_message(channel_id: models::UUIDWrapper, message_id: models::UUIDWrapper, user: User, mut db: Connection<Db>) -> Result<Message, ChannelError> {
    db.get_member(channel_id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })?;

    db.get_message(channel_id.into(), message_id.into())
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })
}

#[post("/channels/<channel_id>/messages", format = "json", data = "<message>")]
pub async fn create_channel_message(channel_id: models::UUIDWrapper, message: models::MessageInsert, user: User, mut db: Connection<Db>) -> Result<Message, ChannelError> {
    db.get_member(channel_id.into(), user.id)
        .await
        .map_err(|e| match e {
            DataRetrievalError::NotFound => ChannelError::NotFound,
            DataRetrievalError::InternalError => ChannelError::InternalServerError,
        })?;

    let message = Message {
        id: uuid::Uuid::new_v4(),
        user_id: user.id,
        channel_id: channel_id.into(),
        content: message.content,
    };


    db.insert_message(message)
        .await
        .map_err(|e| match e {
            _ => ChannelError::InternalServerError,
        })
}