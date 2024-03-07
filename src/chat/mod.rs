mod endpoints;

use std::fmt::Display;
use rocket::http::uri::Origin;


pub trait ChatService {
    fn mount_chat_service<'a, B>(self, base: B) -> Self
        where
            B: TryInto<Origin<'a>> + Clone + Display,
            B::Error: Display;
}

impl ChatService for rocket::Rocket<rocket::Build> {
    fn mount_chat_service<'a, B>(self, base: B) -> Self
        where B: TryInto<Origin<'a>> + Clone + Display, B::Error: Display
    {
        self.mount(base, routes![login::login, login::register, login::me])
    }
}
