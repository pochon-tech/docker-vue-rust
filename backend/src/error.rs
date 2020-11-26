use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;

#[derive(failure::Fail, Debug)]
pub enum Error {
    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

impl ResponseError for Error {
  fn error_response(&self) -> HttpResponse {
    match *self {
      Error::InternalServerError => {
        HttpResponse::InternalServerError().json("Internal Server Error")
      }
    }
  }
}

impl From<PoolError> for Error {
  fn from(_error: PoolError) -> Self {
    Error::InternalServerError
  }
}
