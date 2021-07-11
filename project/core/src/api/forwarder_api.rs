use futures_util::{stream, StreamExt};
use hyper::body::Bytes;
use hyper::client::HttpConnector;
use hyper::{header, Client, Error};
use hyper::{Body, Method, Request, Response};
use routerify::prelude::*;
use std::convert::Infallible;

pub async fn client_request_response(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = req.data::<Client<HttpConnector, Body>>().unwrap().clone();
    let body = String::from(r#"{"name": "Trần Hữu Đức"}"#);
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:1337/forwarder/internal-server-error")
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.clone().into())
        .unwrap();

    let web_res = client.request(req).await.unwrap();
    let after = web_res.into_body();

    // Compare the JSON we sent (before) with what we received (after):
    let before = stream::once(async move {
        let stream_data = format!(
            "<b>POST request body</b>: {}<br><b>Response</b>: ",
            body.clone()
        );
        let result = hyper::body::Bytes::from(stream_data);
        Ok::<Bytes, Error>(result)
    });

    let body = Body::wrap_stream(before.chain(after));
    Ok(Response::new(body))
}
