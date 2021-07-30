#[macro_use]
extern crate log;

use hyper::{Body, Client, Request, Response, Server, StatusCode};
use hyper_microservice::api::forwarder_api::client_request_response;
use hyper_microservice::api::json_data_api::get_json_data_api;

use routerify::ext::RequestExt;
use routerify::{Middleware, RequestInfo, Router, RouterService};
use core::common::logger::init_logger;
use std::convert::Infallible;
use std::env;
use core::prelude::*;


#[tokio::main]
async fn main() -> Result<(), Infallible> {
    init_logger();
    let ip = env::var("APP_IP_ADDRESS").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "33100".into());
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

// Create a `Router<Body, ApiError>` for response body type `hyper::Body`
// and for handler error type `Infallible`.
fn router() -> Router<Body, ApiError> {
    Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .data(Client::new())
        .middleware(Middleware::pre(logger))
        .get("/", |_| async {
            get_ok_json_response(INDEX.into()).map_error_to_api_error()
        })
        .get("/api/json-data", get_json_data_api)
        .post("/api/forwarder", client_request_response)
        .get("/api/forwarder/error", |_| async {
            get_internal_server_error_response(INTERNAL_SERVER_ERROR.into())
                .map_error_to_api_error()
        })
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

// A middleware which logs an http request.
async fn logger(req: Request<Body>) -> Result<Request<Body>, ApiError> {
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
    let err = err.downcast::<ApiError>().unwrap();
    eprintln!("{}", err.to_string());
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}
