use std::convert::Infallible;
use tokio::fs;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use spdlog::prelude::*;
use crate::submit_endpoint::post_submit_endpoint;
use crate::ai_endpoint::post_ai_endpoint;

const BLOCKED_FILES: [&str; 2] = [".env", "server"];

fn get_content_type(path: &str) -> String {
    match std::path::Path::new(path).extension().and_then(|e| e.to_str()) {
        Some("js") => "text/javascript".to_string(),
        Some("json") => "application/json".to_string(),
        Some("css") => "text/css".to_string(),
        Some("svg") => "image/svg+xml".to_string(),
        Some(".txt") => "text/plain".to_string(),
        Some(".png") => "image/png".to_string(),
        Some(".jpg") => "image/jpeg".to_string(),
        Some(".gif") => "image/gif".to_string(),
        Some(".pdf") => "application/pdf".to_string(),
        Some(".woff2") => "font/woff2".to_string(),
        Some(".zip") => "application/zip".to_string(),
        Some(".mp4") => "video/mp4".to_string(),
        Some(".mp3") => "audio/mpeg".to_string(),
        Some("html") => "text/html".to_string(),
        _ => "text/html".to_string()
    }
}

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
            // Handle empty path (ie should be index.html)
            let path = match req.uri().path() {
                "/" => "/index.html",
                _ => req.uri().path()
            };

            // Don't allow user to snipe blocked files
            match BLOCKED_FILES.contains(&path.trim_start_matches('/')) {
                false => {
                    match fs::read(path.trim_start_matches('/')).await {
                        Ok(data) => Response::builder()
                            .status(StatusCode::OK)
                            .header(header::CONTENT_TYPE, get_content_type(path))
                            .body(Body::from(data))
                            .unwrap_or(default_reply),
                        Err(_) => {
                            warn!("Client attempting to get invalid file {:?}", path);
                            Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::from("Not Found"))
                                .unwrap_or(default_reply)
                        },
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

#[allow(unused_imports)]
mod tests {
    use crate::request::get_content_type;

    #[test]
    fn test_content_type() {
        assert_eq!(get_content_type("index.html"), "text/html");
        assert_eq!(get_content_type("index.js"), "text/javascript");
        assert_eq!(get_content_type("index.json"), "application/json");
        assert_eq!(get_content_type("garbage"), "text/html");
    }
}