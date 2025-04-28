use rocket::http::Status;
use rocket::response::Responder;
use diesel::result::Error as DieselError;

#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status, T));  // (HTTP status, data)

#[derive(Responder)]
pub struct ErrorResponse(pub (Status, String));  // (HTTP status, error message)

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;

impl From<DieselError> for ErrorResponse {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => 
                ErrorResponse((Status::NotFound, "Resource not found".to_string())),
            _ => 
                ErrorResponse((Status::InternalServerError, err.to_string())),
        }
    }
}

impl<T> SuccessResponse<T> {
    pub fn map<U, F>(self, f: F) -> SuccessResponse<U>
    where
        F: FnOnce(T) -> U,
    {
        SuccessResponse((self.0.0, f(self.0.1)))
    }
}