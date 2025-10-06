use std::convert::Infallible;
use tokio::fs;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use spdlog::prelude::*;
use crate::submit_endpoint::post_submit_endpoint;
use crate::ai_endpoint::post_ai_endpoint;

const BLOCKED_FILES: [&str; 2] = [".env", "server"];

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = req.method().clone();
    let endpoint = req.uri().clone();
    let default_reply = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap_or_else(|_| Response::new(Body::empty()));

    let mut response = match (method, endpoint.path()) {
        // Handle OPTIONS preflight
        (Method::OPTIONS, _) => Response::builder()
            .status(204)
            .body(Body::empty())
            .unwrap_or(default_reply),

        // Submit code
        (Method::POST, "/submit") => {
            post_submit_endpoint(req).await
        },

        // ai garbage wrapper
        (Method::POST, "/ai") => {
            post_ai_endpoint(req).await
        },

        // Serve website
        (Method::GET, _) => {
            info!("GET request: {:#?}", req);

            // Handle empty path (ie should be index.html)
            let path = match req.uri().path() {
                "/" => "/index.html",
                _ => req.uri().path()
            };

            // Don't allow user to snipe blocked files
            match BLOCKED_FILES.contains(&path.trim_start_matches('/')) {
                false => {
                    info!("Path: {:#?}", path.trim_start_matches('/'));
                    match fs::read(path.trim_start_matches('/')).await {
                        Ok(data) => Response::builder()
                            .status(StatusCode::OK)
                            .body(Body::from(data))
                            .unwrap_or(default_reply),
                        Err(_) => Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("Not Found"))
                            .unwrap_or(default_reply),
                    }
                },
                true => {
                    warn!("Client attempted to grab blocked file \"{}\".", path);
                    Response::builder()
                            .status(StatusCode::FORBIDDEN)
                            .body(Body::from("Forbidden"))
                            .unwrap_or(default_reply)
                }
            }
        },

        // Default 404
        _ => Response::builder()
            .status(404)
            .body(Body::from("Not found"))
            .unwrap_or(default_reply),
    };

    // Add CORS headers
    let headers = response.headers_mut();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, header::HeaderValue::from_static("*"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, header::HeaderValue::from_static("GET, POST, OPTIONS"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, header::HeaderValue::from_static("Content-Type"));

    Ok(response)
}