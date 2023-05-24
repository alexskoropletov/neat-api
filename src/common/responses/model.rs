use crate::common::errors::Error;
use serde::Serialize;
use warp::Rejection;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub status: String,
}

pub type Result<T> = std::result::Result<T, Error>;
pub type WebResult<T> = std::result::Result<T, Rejection>;
