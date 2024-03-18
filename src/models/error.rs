use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::ser::SerializeStruct;
use serde::Serialize;

#[macro_export]
macro_rules! impl_responder_for_error_type {
    ($struct_name:ident) => {
        #[async_trait]
        impl<'r> Responder<'r, 'static> for $struct_name {
            fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
                (self.status(), Json(self.message())).respond_to(request)
            }
        }

        impl<'r> Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
            S: serde::Serializer {
                let mut state = serializer.serialize_struct("Error", 1)?;
                state.serialize_field("message", self.message())?;
                state.end()
            }
        }
    };

    ($struct_name:ident<$lt:lifetime>) => {
        #[async_trait]
        impl<'r> Responder<'r, 'static> for $struct_name<'lr> {
            fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
                (self.status(), Json(self.message())).respond_to(request)
            }
        }

        impl<'r> Serialize for $struct_name<'lr> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
            S: serde::Serializer {
                let mut state = serializer.serialize_struct("Error", 1)?;
                state.serialize_field("message", self.message())?;
                state.end()
            }
        }
    };
}

pub trait Error<'a> {
    fn message(&'a self) -> &'a str;
    fn status(&self) -> Status;
}


impl_responder_for_error_type!(LoginError);
impl_responder_for_error_type!(RegisterError);



pub enum LoginError {
    InternalServerError,
    Unauthorized,
}

impl Error<'_> for LoginError {
    fn message(&'_ self) -> &'_ str {
        match self {
            LoginError::InternalServerError => "Internal Server Error",
            LoginError::Unauthorized => "Invalid credentials",
        }
    }

    fn status(&self) -> Status {
        match self {
            LoginError::InternalServerError => Status::InternalServerError,
            LoginError::Unauthorized => Status::Unauthorized,
        }
    }
}

pub enum RegisterError {
    InternalServerError,
    Conflict,
}

impl Error<'_> for crate::models::error::RegisterError {
    fn message(&'_ self) -> &'_ str {
        match self {
            crate::models::error::RegisterError::InternalServerError => "Internal Server Error",
            crate::models::error::RegisterError::Conflict => "User already exists",
        }
    }

    fn status(&self) -> Status {
        match self {
            crate::models::error::RegisterError::InternalServerError => Status::InternalServerError,
            crate::models::error::RegisterError::Conflict => Status::Conflict,
        }
    }
}

