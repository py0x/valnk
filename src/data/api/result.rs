use serde_dynamo;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid request: `{0}`")]
    BadRequest(String),

    #[error("invalid input data")]
    InvalidInputData(#[source] serde_dynamo::Error),

    #[error("invalid output data")]
    InvalidOutputData(#[source] serde_dynamo::Error),

    #[error("failed to make a request, upstream server error: `{0}`")]
    ServerError(String),

    #[error("unknown error: {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;