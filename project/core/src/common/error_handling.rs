use std::fmt;
use hyper::http::Error;
use hyper::{Body, Response};
use super::http_response::{get_internal_server_error_response, get_unauthorized_response};

// Define a custom error enum to model a possible API service error.
#[derive(Debug)]
pub enum ApiError {
    #[allow(dead_code)]
    Unauthorized,
    Msg(String),
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::Msg(s) => write!(f, "{}", s),
        }
    }
}

impl ApiError {
    pub fn to_response(&self) -> Result<Response<Body>, Error> {
        match self {
            ApiError::Unauthorized => get_unauthorized_response(),
            ApiError::Msg(s) => get_internal_server_error_response((*s).to_string().into()),
        }
    }
}

pub trait ApiErrorHandler {
    fn map_error_to_api_error(self) -> Result<Response<Body>, ApiError>;
}

impl ApiErrorHandler for Result<Response<Body>, hyper::http::Error> {
    fn map_error_to_api_error(self) -> Result<Response<Body>, ApiError> {
        return self.map_err(|err| ApiError::Msg(err.to_string()));
    }
}
