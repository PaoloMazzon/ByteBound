use std::convert::Infallible;
use hyper::{header, Body, Method, Request, Response};
use hyper::body::to_bytes;
use anyhow::anyhow;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ApiRequest {
    constraints: Constraints,
    code: String,
    challenge_name: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Constraints {
    cpu: i64,
    ram: i64,
}

/// Returns a json guaranteed to contain all necessary fields if the request
/// is valid, otherwise returns an error
async fn validate_request(req: Request<Body>) -> Result<ApiRequest, anyhow::Error> {
    let bytes = to_bytes(req.into_body()).await?;
    let string = String::from_utf8(bytes.to_vec())
        .map_err(|e| anyhow!("{:?}", e))?;
    let json: ApiRequest = serde_json::from_str(&string)?;

    Ok(json)
}

fn process_reply(request: ApiRequest) -> Response<Body> {
    // the json passed to this function is guaranteed to be valid

    println!("Request: {:#?}", request);

    Response::new("body".into())
}

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = req.method().clone();
    let json_potential = validate_request(req).await;
    // really trusting this not to explode
    let default_reply = Response::builder().status(500).body("Failed".into()).unwrap();

    let mut response = match method {
        // Handle OPTIONS preflight
        Method::OPTIONS => Response::builder()
            .status(204)
            .body(Body::empty())
            .unwrap_or(default_reply),

        // Example route
        Method::POST => {
            match json_potential {
                Ok(json) => {
                    process_reply(json)
                },
                Err(e) => Response::builder()
                                    .status(400)
                                    .body(Body::from(format!("{:?}", e)))
                                    .unwrap(),
            }
        }

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