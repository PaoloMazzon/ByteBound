use hyper::{Body, Request, Response, StatusCode};
use hyper::body::to_bytes;
use anyhow::anyhow;
use serde::{Serialize, Deserialize};
use dotenvy::dotenv;
use gemini_client_rs::{
    types::{GenerateContentRequest},
    GeminiClient
};
use serde_json::{json};
use spdlog::prelude::*;

/// AI request
#[derive(Deserialize, Debug)]
struct ApiAiRequest {
    /// AI chatbot prompt
    prompt: String
}

/// AI reply
#[derive(Serialize, Debug)]
struct ApiAiReply {
    /// AI chatbot prompt reply
    reply: String
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
async fn validate_ai_request(req: Request<Body>) -> Result<ApiAiRequest, anyhow::Error> {
    let bytes = to_bytes(req.into_body()).await?;
    let string = String::from_utf8(bytes.to_vec())
        .map_err(|e| anyhow!("{:?}", e))?;
    let json: ApiAiRequest = serde_json::from_str(&string)?;

    Ok(json)
}

pub async fn post_ai_endpoint(req: Request<Body>) -> Response<Body> {
    let default_reply = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap_or_else(|_| Response::new(Body::empty())); 
    
    match validate_ai_request(req).await {
        Ok(json) => {
            let reply = serde_json::to_string(&process_ai_reply(json).await).unwrap_or("{}".to_string());
            Response::builder()
                     .status(StatusCode::OK)
                     .body(Body::from(reply))
                     .unwrap_or(default_reply)
        },
        Err(e) => {
            warn!("Received bad /ai request, error: {:?}", e);
            Response::builder()
                     .status(StatusCode::BAD_REQUEST)
                     .body(Body::from(format!("{:?}", e)))
                     .unwrap_or(default_reply)
        }
    }
}