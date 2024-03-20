use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use crate::database::auth::verify_login_token;
use crate::database::Db;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub perms: Permissions,
    pub exp: usize,
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthClaims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("TRY");
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                let token = token.trim_start_matches("BEARER ");
                match verify_login_token(token) {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Permissions(i32);

// developer
// identify
// get channels
// join/leave channels
// modify/create/delete channels
// see other users
// see messages
// modify/create/delete messages
// add members
// kick members
// ban members
// modify members

macro_rules! get_bit {
    ($val:expr, $n:expr) => { $val & (1 << $n) == (1 << $n) };
}

impl Permissions {
    pub fn new(
        developer: bool,
        identify: bool,
        get_channels: bool,
        join_leave_channels: bool,
        modify_create_delete_channels: bool,
        see_other_users: bool,
        see_messages: bool,
        modify_create_delete_messages: bool,
        add_members: bool,
        kick_members: bool,
        ban_members: bool,
        modify_members: bool
    ) -> Self {
        Self(
            if developer                      { 1 << 0 } else { 0 } |
                if identify                      { 1 << 1 } else { 0 } |
                if get_channels                  { 1 << 2 } else { 0 } |
                if join_leave_channels           { 1 << 3 } else { 0 } |
                if modify_create_delete_channels { 1 << 4 } else { 0 } |
                if see_other_users               { 1 << 5 } else { 0 } |
                if see_messages                  { 1 << 6 } else { 0 } |
                if modify_create_delete_messages { 1 << 7 } else { 0 } |
                if add_members                   { 1 << 8 } else { 0 } |
                if kick_members                  { 1 << 9 } else { 0 } |
                if ban_members                   { 1 << 10 } else { 0 } |
                if modify_members                { 1 << 11 } else { 0 }
        )
    }

    pub fn developer(&self) -> bool { get_bit!(&self.0, 0) }
    pub fn identify(&self) -> bool { get_bit!(&self.0, 1) }
    pub fn get_channels(&self) -> bool { get_bit!(&self.0, 2) }
    pub fn join_leave_channels(&self) -> bool { get_bit!(&self.0, 3) }
    pub fn modify_create_delete_channels(&self) -> bool { get_bit!(&self.0, 4) }
    pub fn see_other_users(&self) -> bool { get_bit!(&self.0, 5) }
    pub fn see_messages(&self) -> bool { get_bit!(&self.0, 6) }
    pub fn modify_create_delete_messages(&self) -> bool { get_bit!(&self.0, 7) }
    pub fn add_members(&self) -> bool { get_bit!(&self.0, 8) }
    pub fn kick_members(&self) -> bool { get_bit!(&self.0, 9) }
    pub fn ban_members(&self) -> bool { get_bit!(&self.0, 10) }
    pub fn modify_members(&self) -> bool { get_bit!(&self.0, 11) }
}