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
        self.mount(base, routes![endpoints::get_channel_by_id, endpoints::patch_channel_by_id, endpoints::create_channel, endpoints::remove_channel_by_id])
    }
}