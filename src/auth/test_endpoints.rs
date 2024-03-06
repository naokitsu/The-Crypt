#[post("/", format = "json", data = "<login_request>")]
pub async fn login(login_request: Json<models::LoginRequest<'_>>, mut db: Connection<Db>) -> Result<models::Token, models::LoginError> {
    db.login(login_request.username, login_request.password).await
}
