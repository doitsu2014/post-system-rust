use core::api::forwarder_api::client_request_response;
use core::static_data::{INDEX, INTERNAL_SERVER_ERROR, NOT_FOUND};
use core::GenericError;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, Server, StatusCode};

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
        (&Method::GET, "/internal-server-error") => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap()),
        (&Method::POST, "/forwarder") => client_request_response(&client).await,
        (&Method::POST, "/forwarder/internal-server-error") => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(r#"{"message": "Internal server error messages"}"#.into())
            .unwrap()),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOT_FOUND.into())
                .unwrap())
        }
    }
}
