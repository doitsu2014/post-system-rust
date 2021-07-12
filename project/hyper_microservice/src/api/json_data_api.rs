use core::prelude::*;
use hyper::body::Buf;
use hyper::{Body, Request, Response};

pub async fn get_json_data_api(_: Request<Body>) -> Result<Response<Body>, ApiError> {
    let data = vec!["foo", "bar"];
    (match serde_json::to_string(&data) {
        Ok(json) => get_ok_json_response(Body::from(json)),
        Err(_) => get_internal_server_error_response(INTERNAL_SERVER_ERROR.into()),
    })
    .map_error_to_api_error()
}

pub async fn post_json_data_api(req: Request<Body>) -> Result<Response<Body>, ApiError> {
    let whole_request_body = hyper::body::aggregate(req).await.unwrap();
    let mut data: serde_json::Value = serde_json::from_reader(whole_request_body.reader()).unwrap();
    data["test"] = serde_json::Value::from("test value");
    let json = serde_json::to_string(&data).unwrap();
    
    get_ok_json_response(json.into())
        .map_error_to_api_error()
}
