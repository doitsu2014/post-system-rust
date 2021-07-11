use crate::static_data::NOT_FOUND;
use hyper::{header, Body, Response, StatusCode};
use std::convert::Infallible;

pub fn get_internal_server_error_response<T>(message: T) -> Result<Response<T>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(message.into())
        .unwrap())
}

pub fn get_ok_json_response<T>(body: T) -> Result<Response<T>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.into())
        .unwrap())
}

pub fn get_created_json_response<T>(body: T) -> Result<Response<T>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.into())
        .unwrap())
}

pub fn get_accepted_json_response<T>(body: T) -> Result<Response<T>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::ACCEPTED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.into())
        .unwrap())
}

pub fn get_no_content_response() -> Result<Response<()>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::ACCEPTED)
        .body(())
        .unwrap())
}

pub fn get_not_found_response() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOT_FOUND.into())
        .unwrap())
}
