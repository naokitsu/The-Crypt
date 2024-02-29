use rocket::http::{ContentType, Header, Status};
use rocket::fs::NamedFile;

#[derive(Responder)]
pub enum Error {
    #[response(status = 500, content_type = "json")]
    InternalServerError(String),

    #[response(status = 401, content_type = "json")]
    Unauthorized(String),

    #[response(status = 400, content_type = "json")]
    BadRequest(String),

}