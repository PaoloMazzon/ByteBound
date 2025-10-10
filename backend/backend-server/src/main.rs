mod request;
mod compile;
mod run_container;
mod submit_endpoint;
mod ai_endpoint;
mod output;
mod client_workspace;

use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Notify;
use spdlog::prelude::*;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let shutdown = Arc::new(Notify::new());
    let shutdown_clone = shutdown.clone();
    tokio::spawn(async move {
        let mut term = signal(SignalKind::terminate()).unwrap();
        let mut int = signal(SignalKind::interrupt()).unwrap();
        tokio::select! {
            _ = term.recv() => {},
            _ = int.recv() => {},
        }
        shutdown_clone.notify_one();
        info!("Graceful shutdown signal received");
    });

    // address to bind to
    let addr = ([0, 0, 0, 0], 80).into();

    // a MakeService closure — constructs a new service for each connection
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our async function into a `Service`
        Ok::<_, Infallible>(service_fn(request::handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc)
        .with_graceful_shutdown(shutdown.notified());

    info!("Listening on {}", addr);

    // run the server until Ctrl+C
    server.await?;

    Ok(())
}
