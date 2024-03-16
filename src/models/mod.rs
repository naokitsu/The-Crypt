mod user;
mod channel;
mod message;

trait Model {
    type Patch;
    type Insert;
    type Vector;

    fn to_patch(&self) -> Self::Patch;
    fn to_insert(&self) -> Self::Insert;
}

#[macro_export]
macro_rules! impl_from_data_json_for {
        ($struct_name:ident) => {
        #[async_trait]
        impl<'r> rocket::data::FromData<'r> for $struct_name {
            type Error = rocket::serde::json::Error<'r>;

            async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
                use rocket::serde::json::Json;
                Json::from_data(req, data).await.map(|json: Json<Self>| json.into_inner())
            }
        }
    };

    ($struct_name:ident<$lt:lifetime>) => {
        #[async_trait]
        impl<'r> rocket::data::FromData<'r> for $struct_name<'r> {
            type Error = rocket::serde::json::Error<'r>;

            async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
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


