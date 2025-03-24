pub mod user_controller;
pub mod contact_controller;
pub mod token_verification;
pub mod google_login;

use rocket::http::Status;
use sea_orm::DbErr;

#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status,T));

#[derive(Responder)]
pub struct ErrorResponse(pub (Status,String));      

pub type Response<T>=Result<SuccessResponse<T>,ErrorResponse>;

impl From<DbErr> for ErrorResponse{
    fn from(err:DbErr)->Self{
        ErrorResponse((Status::InternalServerError,err.to_string()))
    }
}