mod request;

use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // address to bind to
    let addr = ([127, 0, 0, 1], 3000).into();

    // a MakeService closure — constructs a new service for each connection
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our async function into a `Service`
        Ok::<_, Infallible>(service_fn(request::handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // run the server until Ctrl+C
    server.await?;

    Ok(())
}
