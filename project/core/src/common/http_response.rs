use crate::static_data::{NOT_FOUND, UNAUTHORIZED};
use hyper::http::Error;
use hyper::{header, Body, Response, StatusCode};

pub fn get_internal_server_error_response<T>(message: T) -> Result<Response<T>, Error> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(message.into())
}

pub fn get_ok_json_response<T>(body: T) -> Result<Response<T>, Error> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.into())
}

pub fn get_created_json_response<T>(body: T) -> Result<Response<T>, Error> {
    Response::builder()
        .status(StatusCode::CREATED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.into())
}

pub fn get_accepted_json_response<T>(body: T) -> Result<Response<T>, Error> {
    Response::builder()
        .status(StatusCode::ACCEPTED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.into())
}

pub fn get_no_content_response() -> Result<Response<()>, Error> {
    Response::builder().status(StatusCode::ACCEPTED).body(())
}

pub fn get_not_found_response() -> Result<Response<Body>, Error> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOT_FOUND.into())
}

pub fn get_unauthorized_response() -> Result<Response<Body>, Error> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(UNAUTHORIZED.into())
}
