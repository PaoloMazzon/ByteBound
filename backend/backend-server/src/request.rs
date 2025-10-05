use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("{:?}", req);
    Ok(Response::new(Body::from("Hello, world!")))
}