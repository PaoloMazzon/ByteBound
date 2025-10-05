use std::convert::Infallible;
use hyper::{header, Body, Method, Request, Response};
use hyper::body::to_bytes;
use anyhow::anyhow;
use serde::{Serialize, Deserialize};
use dotenvy::dotenv;
use gemini_client_rs::{
    types::{GenerateContentRequest},
    GeminiClient
};
use serde_json::{json};

/// AI request
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ApiAiRequest {
    /// AI chatbot prompt
    prompt: String
}

/// AI reply
#[allow(dead_code)]
#[derive(Serialize, Debug)]
struct ApiAiReply {
    /// AI chatbot prompt reply
    reply: String
}

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
    // TODO: Actually parse it
    println!("Request: {:#?}", request);

    ApiReply { 
        compiled: true, 
        success: true, 
        runtime_us: 1000, 
        errors: "".to_string(), 
        test_cases: vec!()
    }
}

/// Slightly less top-level ai processing function
async fn process_ai_reply_less(request: ApiAiRequest) -> Result<ApiAiReply, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("GEMINI_API_KEY")?;
    let client = GeminiClient::new(api_key);
    let model_name = "gemini-2.5-flash";
    
    // Create a single request with just the user's message
    let req_json = json!({
        "contents": [{
            "parts": [{"text": request.prompt}],
            "role": "user"
        }]
    });
    
    let request: GenerateContentRequest = serde_json::from_value(req_json)?;
    let response = client.generate_content(model_name, &request).await?;
    
    // Extract the text response
    let mut response_text = String::new();
    for candidate in &response.candidates {
        for part in &candidate.content.parts {
            let s = serde_json::to_string(&part.data).unwrap_or("".to_string());
            response_text.push_str(&s);
        }
    }
    
    // Return the AI response
    Ok(ApiAiReply {
        reply: response_text,
    })
}

/// Top-level ai processing function
async fn process_ai_reply(request: ApiAiRequest) -> ApiAiReply {
    process_ai_reply_less(request).await.unwrap_or(ApiAiReply { reply: "Failed to connect to Gemini.".to_string() })
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

/// Returns a json guaranteed to contain all necessary fields if the request
/// is valid, otherwise returns an error
async fn validate_ai_request(req: Request<Body>) -> Result<ApiAiRequest, anyhow::Error> {
    let bytes = to_bytes(req.into_body()).await?;
    let string = String::from_utf8(bytes.to_vec())
        .map_err(|e| anyhow!("{:?}", e))?;
    let json: ApiAiRequest = serde_json::from_str(&string)?;

    Ok(json)
}

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = req.method().clone();
    let endpoint = req.uri().clone();

    // really trusting this not to explode
    let default_reply = Response::builder().status(500).body("Failed".into()).unwrap();

    let mut response = match (method, endpoint.path()) {
        // Handle OPTIONS preflight
        (Method::OPTIONS, _) => Response::builder()
            .status(204)
            .body(Body::empty())
            .unwrap_or(default_reply),

        // Submit code
        (Method::POST, "submit") => {
            match validate_request(req).await {
                Ok(json) => {
                    let reply = process_reply(json);
                    let body = match serde_json::to_string(&reply) {
                        Ok(string) => string,
                        Err(e) => {
                            println!("Failed to parse submit request {:#?}, {:?}", reply, e);
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

        // ai garbage wrapper
        (Method::POST, "ai") => {
            match validate_ai_request(req).await {
                Ok(json) => {
                    let reply = process_ai_reply(json).await;
                    let body = match serde_json::to_string(&reply) {
                        Ok(string) => string,
                        Err(e) => {
                            println!("Failed to parse ai request {:#?}, {:?}", reply, e);
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