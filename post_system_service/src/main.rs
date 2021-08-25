use std::io;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};
use warp::Filter;
use post_system_service::abstraction::Setting;

#[tokio::main()]
async fn main() {
    let setting = Setting::new();
    let file_appender =
        tracing_appender::rolling::daily(setting.log_file_path.to_owned(), "post_system_rust.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::Layer::new().with_writer(io::stdout))
        .with(fmt::Layer::new().with_writer(non_blocking).json());
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global collector");

    let hello = warp::path("hello")
        .and(warp::get())
        // When the `hello` route is called, emit a `tracing` event.
        .map(|| {
            tracing::info!("saying hello...");
            "Hello, World!"
        })
        // Wrap the route in a `tracing` span to add the route's name as context
        // to any events that occur inside it.
        .with(warp::trace::named("hello"));

    let goodbye = warp::path("goodbye")
        .and(warp::get())
        .map(|| {
            tracing::info!("saying goodbye...");
            "So long and thanks for all the fish!"
        })
        // We can also provide our own custom `tracing` spans to wrap a route.
        .with(warp::trace(|info| {
            // Construct our own custom span for this route.
            tracing::info_span!("goodbye", req.path = ?info.path())
        }));

    let routes = hello
        .or(goodbye)
        // Wrap all the routes with a filter that creates a `tracing` span for
        // each request we receive, including data about the request.
        .with(warp::trace::request());

    tracing::info!("{:?}", setting);
    match setting.tls {
        true => {
            warp::serve(routes)
                .tls()
                .key_path(&setting.tls_key_path)
                .cert_path(&setting.tls_cert_path)
                .run(([127, 0, 0, 1], 3030))
                .await
        }
        false => warp::serve(routes).run(([127, 0, 0, 1], 3030)).await,
    }
}
