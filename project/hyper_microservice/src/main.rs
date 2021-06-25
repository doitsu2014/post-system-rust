use futures::TryStreamExt as _;
use hyper::header::HeaderValue;
use hyper::header::CONTENT_TYPE;
use hyper::header::HOST;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr)
        .serve(make_svc)
        .with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            *response.body_mut() = Body::wrap_stream(mapping);
        }
        (&Method::POST, "/echo/reverse") => {
            let full_body = hyper::body::to_bytes(req.into_body()).await?;

            *response.body_mut() = 
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    Ok(response)
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
