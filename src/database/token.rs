use diesel::result::Error;

pub(crate) enum DataRetrievalError {
    NotFound,
    InternalError,
}

pub(crate) enum DataInsertionError {
    AlreadyExists,
    InternalError,
}

pub(crate) enum DataSetError {
    InvalidSession,
    InternalError,
}

pub(crate) enum DataRemovalError {
    InvalidSession,
    InternalError,
}


pub trait Database {
    type Id<'a>;

    async fn get_public_key(&mut self, user: Self::Id<'_>) -> Result<[u8; 32], DataRetrievalError>;

    async fn get_session_nonce(&mut self, user: Self::Id<'_>, key: [u8; 32]) -> Result<[u8; 32], DataRetrievalError>;

    async fn set_session_nonce(
        &mut self,
        user: Self::Id<'_>,
        key: [u8; 32],
        nonce: [u8; 32],
    ) -> Result<(), DataSetError>;

    async fn insert_session(
        &mut self,
        user: Self::Id<'_>,
        key: [u8; 32],
        nonce: [u8; 32],
    ) -> Result<(), DataInsertionError>;

    async fn remove_session(&mut self, user: Self::Id<'_>, key: [u8; 32]) -> Result<(), DataRemovalError>;
}

impl Database for rocket_db_pools::Connection<crate::database::Db> {
    type Id<'a> = uuid::Uuid;

    async fn get_public_key(&mut self, user: Self::Id<'_>) -> Result<[u8; 32], DataRetrievalError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::sessions;

        sessions::table
            .select(sessions::key)
            .filter(sessions::user_id.eq(user))
            .first::<Vec<u8>>(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataRetrievalError::NotFound,
                _ => DataRetrievalError::InternalError,
            })
            .and_then(|vec| {
                let mut fixed_array = [0u8; 32];
                fixed_array.copy_from_slice(&vec[..]);
                Ok(fixed_array)
            })
    }

    async fn get_session_nonce(&mut self, user: Self::Id<'_>, key: [u8; 32]) -> Result<[u8; 32], DataRetrievalError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::sessions;

        sessions::table
            .select(sessions::nonce)
            .filter(sessions::user_id.eq(user))
            .filter(sessions::key.eq(key.as_slice()))
            .first::<Vec<u8>>(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataRetrievalError::NotFound,
                _ => DataRetrievalError::InternalError,
            })
            .and_then(|vec| {
                let mut fixed_array = [0u8; 32];
                fixed_array.copy_from_slice(&vec[..]);
                Ok(fixed_array)
            })
    }

    async fn set_session_nonce(&mut self, user: Self::Id<'_>, key: [u8; 32], nonce: [u8; 32]) -> Result<(), DataSetError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::sessions;

        diesel::update(sessions::table)
            .filter(sessions::user_id.eq(user))
            .filter(sessions::key.eq(key.as_slice()))
            .set(sessions::nonce.eq(nonce.as_slice()))
            .execute(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataSetError::InvalidSession,
                _ => DataSetError::InternalError,
            })
            .map(|_| ())
    }

    async fn insert_session(&mut self, user: Self::Id<'_>, key: [u8; 32], nonce: [u8; 32]) -> Result<(), DataInsertionError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::sessions;

        diesel::insert_into(sessions::table)
            .values((
                sessions::key.eq(key.as_slice()),
                sessions::nonce.eq(nonce.as_slice()),
                sessions::user_id.eq(user)
            ))
            .execute(self)
            .await
            .map_err(|e| match e {
                Error::DatabaseError(_, _) => DataInsertionError::AlreadyExists,
                _ => DataInsertionError::InternalError,
            })
            .map(|_| ())
    }

    async fn remove_session(&mut self, user: Self::Id<'_>, key: [u8; 32]) -> Result<(), DataRemovalError> {
        use rocket_db_pools::diesel::prelude::*;
        use crate::schema::sessions;

        diesel::delete(sessions::table)
            .filter(sessions::user_id.eq(user))
            .filter(sessions::key.eq(key.as_slice()))
            .execute(self)
            .await
            .map_err(|e| match e {
                Error::NotFound => DataRemovalError::InvalidSession,
                _ => DataRemovalError::InternalError,
            })
            .map(|_| ())
    }
}