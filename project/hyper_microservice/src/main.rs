use core::api::forwarder_api::client_request_response;
use core::api::json_data_api::{get_json_data_api, post_json_data_api};
use core::common::http_response::{get_internal_server_error_response, get_not_found_response};
use core::static_data::{INDEX, INTERNAL_SERVER_ERROR};
use core::GenericError;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, Server};

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    pretty_env_logger::init();
    let addr = "127.0.0.1:1337".parse().unwrap();

    let client = Client::new();
    let new_service = make_service_fn(move |_| {
        let client = client.clone();
        async { Ok::<_, GenericError>(service_fn(move |req| forward_req(req, client.to_owned()))) }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}

async fn forward_req(
    req: Request<Body>,
    client: Client<HttpConnector>,
) -> Result<Response<Body>, GenericError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(INDEX.into())),
        (&Method::GET, "/api/json-data") => get_json_data_api().await,
        (&Method::POST, "/api/json-data") => post_json_data_api(req).await,
        (&Method::GET, "/internal-server-error") => get_internal_server_error_response(INTERNAL_SERVER_ERROR.into()),
        (&Method::POST, "/forwarder") => client_request_response(&client).await,
        _ => get_not_found_response()
    }
}
