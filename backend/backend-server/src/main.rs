use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, world!")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // address to bind to
    let addr = ([127, 0, 0, 1], 3000).into();

    // a MakeService closure — constructs a new service for each connection
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our async function into a `Service`
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // run the server until Ctrl+C
    server.await?;

    Ok(())
}
