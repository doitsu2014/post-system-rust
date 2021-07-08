extern crate serde_json;
use crate::common::http_response::{get_internal_server_error_response, get_ok_json_response};
use crate::static_data::INTERNAL_SERVER_ERROR;
use crate::GenericError;
use hyper::body::Buf;
use hyper::{Body, Request, Response};

pub async fn get_json_data_api() -> Result<Response<Body>, GenericError> {
    let data = vec!["foo", "bar"];
    match serde_json::to_string(&data) {
        Ok(json) => get_ok_json_response(Body::from(json)),
        Err(_) => get_internal_server_error_response(INTERNAL_SERVER_ERROR.into()),
    }
}

pub async fn post_json_data_api(req: Request<Body>) -> Result<Response<Body>, GenericError> {
    let whole_request_body = hyper::body::aggregate(req).await?;
    let mut data: serde_json::Value = serde_json::from_reader(whole_request_body.reader())?;
    data["test"] = serde_json::Value::from("test value");
    let json = serde_json::to_string(&data)?;

    get_ok_json_response(json.into())
}
