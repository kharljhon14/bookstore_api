use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};
use sea_orm::DbErr;

pub mod auth;
pub mod author;
pub mod book;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResError {
    message: String,
}

#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status, T));

#[derive(Responder)]
pub struct ErrorResponse(pub (Status, Json<ResError>));

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;

impl From<DbErr> for ErrorResponse {
    fn from(err: DbErr) -> Self {
        ErrorResponse((
            Status::InternalServerError,
            Json(ResError {
                message: err.to_string(),
            }),
        ))
    }
}
