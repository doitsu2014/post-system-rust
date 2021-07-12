use crate::common::error_handling::ApiError;

use hyper::client::HttpConnector;
use hyper::{header, Client};
use hyper::{Body, Method, Request, Response};
use routerify::prelude::*;

pub async fn client_request_response(req: Request<Body>) -> Result<Response<Body>, ApiError> {
    let client = match req.data::<Client<HttpConnector, Body>>() {
        Some(c) => c.clone(),
        None => return Err(ApiError::Msg("client instance is missing".into()))
    };

    let body = String::from(r#"{"name": "Trần Hữu Đức"}"#);
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:1337/forwarder/internal-server-error")
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.clone().into())
        .unwrap();

    let web_res = client.request(req).await.unwrap();
    Ok(Response::new(web_res.into_body()))
}
