mod user;
mod channel;
mod message;
mod member;
mod channel_ban;
mod login_request;
mod register_request;
mod token;
mod uuid;
mod error;
mod permissions;

trait Model {
    type Patch;
    type Insert;
    type Vector;

    fn to_patch(&self) -> Self::Patch;
    fn to_insert(&self) -> Self::Insert;
}

// --- exports ---

pub use user::User;
pub use user::insert::Insert as UserInsert;
pub use user::patch::Patch as UserPatch;
pub use user::users::Users;

pub use channel::Channel;
pub use channel::insert::Insert as ChannelInsert;
pub use channel::patch::Patch as ChannelPatch;
pub use channel::channels::Channels;

pub use message::Message;
pub use message::insert::Insert as MessageInsert;
pub use message::patch::Patch as MessagePatch;
pub use message::messages::Messages;

pub use member::Member;
pub use member::insert::Insert as MemberInsert;
pub use member::patch::Patch as MemberPatch;
pub use member::members::Members;

pub use channel_ban::ChannelBan;
pub use channel_ban::channel_bans::ChannelBans;

pub use login_request::LoginRequest;
pub use register_request::RegisterRequest;
pub use token::Token;
pub use uuid::UUIDWrapper;

pub use error::Error;
pub use error::LoginError;
pub use error::RegisterError;

pub use permissions::AuthClaims;
pub use permissions::Permissions;

// --- Macros---
#[macro_export]
macro_rules! impl_from_data_json_for {
        ($struct_name:ident) => {
        #[async_trait]
        impl<'r> rocket::data::FromData<'r> for $struct_name {
            type Error = rocket::serde::json::Error<'r>;

            async fn from_data(req: &'r rocket::Request<'_>, data: rocket::data::Data<'r>) -> rocket::data::Outcome<'r, Self> {
                use rocket::serde::json::Json;
                Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
            }
        }
    };

    ($struct_name:ident<$lt:lifetime>) => {
        #[async_trait]
        impl<'r> rocket::data::FromData<'r> for $struct_name<'r> {
            type Error = rocket::serde::json::Error<'r>;

            async fn from_data(req: &'r rocket::Request<'_>, data: rocket::data::Data<'r>) -> rocket::data::Outcome<'r, Self> {
                use rocket::serde::json::Json;
                Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_responder_json_for {
    ($struct_name:ident) => {
        #[async_trait]
        impl<'lt> rocket::response::Responder<'lt, 'lt> for $struct_name {
            fn respond_to(self, request: &'lt rocket::Request<'_>) -> rocket::response::Result<'lt> {
                rocket::serde::json::Json(self).respond_to(request)
            }
        }
    };

    ($struct_name:ident<$lt:lifetime>) => {
        #[async_trait]
        impl<'lt> rocket::response::Responder<'lt, 'lt> for $struct_name<'lt> {
            fn respond_to(self, request: &'lt rocket::Request<'_>) -> rocket::response::Result<'lt> {
                rocket::serde::json::Json(self).respond_to(request)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deserialize_for_vector_wrapper {
    ($struct_name:ident, $inner:ident) => {
        impl<'de> Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
                Ok($struct_name(Vec::<$inner>::deserialize(deserializer)?))
            }
        }
    };

    ($struct_name:ident<$lt:lifetime>, $inner:ident) => {
        impl<'lt> Deserialize<'lt> for $struct_name<'lt> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'lt> {
                Ok($struct_name(Vec::<$inner>::deserialize(deserializer)?))
            }
        }
    };
}
