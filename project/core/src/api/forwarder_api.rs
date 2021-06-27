use crate::{GenericError};
use hyper::body::Bytes;
use hyper::client::HttpConnector;
use hyper::{Error, header};
use hyper::{Body, Client, Method, Request, Response};
use futures_util::{stream, StreamExt};

pub async fn client_request_response(
	client: &Client<HttpConnector>,
) -> Result<Response<Body>, GenericError> {
	let body = String::from(r#"{"name": "Trần Hữu Đức"}"#);
	let req = Request::builder()
		.method(Method::POST)
		.uri("http://localhost:1337/forwarder/internal-server-error")
		.header(header::CONTENT_TYPE, "application/json")
		.body(body.clone().into())
		.unwrap();

	let web_res = client.request(req).await?;

	// Compare the JSON we sent (before) with what we received (after):
	let before = stream::once(async move {
		let stream_data = format!(
			"<b>POST request body</b>: {}<br><b>Response</b>: ",
			body.clone()
		);
		let result = hyper::body::Bytes::from(stream_data);
		Ok::<Bytes, Error>(result)
	});

	let after = web_res.into_body();
	let body = Body::wrap_stream(before.chain(after));

	Ok(Response::new(body))
}
