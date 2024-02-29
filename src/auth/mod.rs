mod login;
mod objects;

use std::fmt;
use std::fmt::Display;
use rocket::Build;
use rocket::http::uri::Origin;


pub trait AuthService {
    fn mount_auth_service<'a, B>(self, base: B) -> Self
        where
            B: TryInto<Origin<'a>> + Clone + Display,
            B::Error: Display;
}

impl AuthService for rocket::Rocket<Build> {
    fn mount_auth_service<'a, B>(self, base: B) -> Self
        where B: TryInto<Origin<'a>> + Clone + Display, B::Error: Display
    {
        self.mount(base, routes![login::login])
    }
}