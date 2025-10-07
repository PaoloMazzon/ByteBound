mod request;
mod compile;
mod run_container;
mod submit_endpoint;
mod ai_endpoint;
mod output;

use spdlog::prelude::*;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // address to bind to
    let addr = ([0, 0, 0, 0], 80).into();

    // a MakeService closure — constructs a new service for each connection
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our async function into a `Service`
        Ok::<_, Infallible>(service_fn(request::handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening on http://{}", addr);

    // run the server until Ctrl+C
    server.await?;

    Ok(())
}
