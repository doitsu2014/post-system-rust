extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use core::api::forwarder_api::client_request_response;
use core::api::json_data_api::{get_json_data_api, post_json_data_api};
use core::common::http_response::{
    get_internal_server_error_response, get_not_found_response, get_ok_json_response,
};
use core::static_data::{INDEX, INTERNAL_SERVER_ERROR};
use core::GenericError;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, Server, StatusCode};
use routerify::ext::RequestExt;
use routerify::{Middleware, RequestInfo, Router, RouterService};
use std::convert::Infallible;
use std::env;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    pretty_env_logger::init();
    let ip = env::var("APP_IP_ADDRESS").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "1337".into());
    let addr = format!("{}:{}", ip, port).parse().unwrap();

    let server = Server::bind(&addr).serve(RouterService::new(router()).unwrap());

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    info!("Application is listenning on http://{}", addr);

    if let Err(e) = graceful.await {
        error!("exception: {}", e);
    }
    Ok(())
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

// Create a `Router<Body, Infallible>` for response body type `hyper::Body`
// and for handler error type `Infallible`.
fn router() -> Router<Body, Infallible> {
    Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .data(Client::new())
        .middleware(Middleware::pre(logger))
        .get("/", |_| async { get_ok_json_response(INDEX.into()) })
        .get("/api/json-data", get_json_data_api)
        .get("/api/forwarder", client_request_response)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

// A middleware which logs an http request.
async fn logger(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

// Define an error handler function which will accept the `routerify::Error`
// and the request information and generates an appropriate response.
async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    eprintln!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

// async fn forward_req(
//     req: Request<Body>,
//     client: Client<HttpConnector>,
// ) -> Result<Response<Body>, GenericError> {
//     match (req.method(), req.uri().path()) {
//         (&Method::GET, "/") => Ok(Response::new(INDEX.into())),
//         (&Method::GET, "/api/json-data") => get_json_data_api().await,
//         (&Method::POST, "/api/json-data") => post_json_data_api(req).await,
//         (&Method::GET, "/internal-server-error") => {
//             get_internal_server_error_response(INTERNAL_SERVER_ERROR.into())
//         }
//         (&Method::POST, "/forwarder") => client_request_response(&client).await,
//         _ => get_not_found_response(),
//     }
// }
