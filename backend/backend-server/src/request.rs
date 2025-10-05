use std::convert::Infallible;
use hyper::{header, Body, Method, Request, Response};
use hyper::body::to_bytes;
use anyhow::anyhow;
use serde::{Serialize, Deserialize};

/// Only needed by ApiRequest
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Constraints {
    /// CPU constraint in mhz
    pub cpu: i64,

    /// RAM amount in bytes
    pub ram: i64,
}

/// Represents a client-provided REST API request, maps 1:1 to REST.md
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ApiRequest {
    /// Cpu and ram constraints
    pub constraints: Constraints,

    /// C code as a string
    pub code: String,

    /// Challenge name to run the code against
    pub challenge_name: String,
}

/// Represents the outgoing response, maps 1:1 to REST.md
#[allow(dead_code)]
#[derive(Serialize, Debug)]
struct ApiReply {
    /// Whether or not the code successfully compiled
    pub compiled: bool,

    /// Whether or not the code ran successfully (not if it passed test cases)
    pub success: bool,

    /// How long it took to run the program inside the docker container (microseconds)
    pub runtime_us: u64,

    /// Any compiler or runtime errors
    pub errors: String,

    /// Whether or not each test case passed
    pub test_cases: Vec<bool>
}

/// Top-level function that gets an api request
fn process_reply(request: ApiRequest) -> ApiReply {
    // the json passed to this function is guaranteed to be valid

    println!("Request: {:#?}", request);

    ApiReply { 
        compiled: true, 
        success: true, 
        runtime_us: 1000, 
        errors: "".to_string(), 
        test_cases: vec!()
    }
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
                    let reply = process_reply(json);
                    let body = match serde_json::to_string(&reply) {
                        Ok(string) => string,
                        Err(e) => {
                            println!("Failed to parse json {:#?}, {:?}", reply, e);
                            "null".to_string()
                        }
                    };

                    Response::builder()
                             .status(200)
                             .body(body.into())
                             .unwrap_or(default_reply)
                },
                Err(e) => Response::builder()
                                    .status(400)
                                    .body(Body::from(format!("{:?}", e)))
                                    .unwrap_or(default_reply),
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