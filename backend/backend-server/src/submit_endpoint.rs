use std::{fs, sync::atomic::{AtomicUsize, Ordering}};
use hyper::{body::to_bytes, Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use spdlog::prelude::*;
use crate::{compile::compile_c_file, run_container::create_runner_safe};

// for unique filenames
static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Only needed by ApiRequest
#[derive(Deserialize, Debug)]
struct Constraints {
    /// CPU constraint in mhz
    pub cpu: i64,

    /// RAM amount in bytes
    pub ram: i64,
}

/// Represents a client-provided REST API request, maps 1:1 to REST.md
#[derive(Deserialize, Debug)]
struct ApiRequest {
    /// Cpu and ram constraints
    pub constraints: Constraints,

    /// C code as a string
    pub code: String,

    /// Challenge name to run the code against
    #[allow(dead_code)]
    pub challenge_name: String,
}

/// Information from running the compiler
#[derive(Serialize, Debug)]
struct ApiCompilerInfo {
    /// Whether or not it successfully compiled
    success: bool,

    /// stdout output from the compiler
    stdout: String,

    /// stderr output from the compiler
    stderr: String
}

/// Information from running the runner container
#[derive(Serialize, Debug)]
struct ApiRunnerInfo {
    /// Whether or not it successfully ran
    success: bool,

    /// stdout output from the runner container
    stdout: String,

    /// stderr output from the runner container
    stderr: String,

    /// Runtime of the program in microseconds
    runtime_us: u64
}

/// Represents the outgoing response, maps 1:1 to REST.md
#[derive(Serialize, Debug)]
struct ApiReply {
    /// Info from compiling the program
    pub compiler: ApiCompilerInfo,
    
    /// Info from running the program
    pub runner: ApiRunnerInfo,

    /// Whether or not each test case passed
    pub test_cases: Vec<bool>
}

impl ApiReply {
    /// Returns a new ApiReply with sensible defaults
    pub fn blank() -> Self {
        ApiReply {
            runner: ApiRunnerInfo {
                success: false,
                stdout: String::new(),
                stderr: String::new(),
                runtime_us: 0,
            },
            compiler: ApiCompilerInfo { 
                success: false, 
                stdout: String::new(), 
                stderr: String::new() 
            },
            test_cases: vec!()
        }
    }
}

/// Top-level function that gets an api request
fn process_reply(request: ApiRequest) -> ApiReply {
    // Attempt to compile the code
    let c_filename = format!("/tmp/garbage{}.c", COUNTER.fetch_add(1, Ordering::SeqCst));
    if let Err(e) = fs::write(c_filename.clone(), request.code.clone()) {
        let mut reply = ApiReply::blank();
        reply.compiler.stderr = format!("{:?}", e);
        return reply;
    }


    let (path, compiler_output) = match compile_c_file(&c_filename.as_str(), format!("/tmp/garbage{}.c", COUNTER.fetch_add(1, Ordering::SeqCst)).as_str()) {
        Ok(path) => path,
        Err(e) => {
            debug!("Failed to compile {:?}", request.code);
            let mut reply = ApiReply::blank();
            reply.compiler.stderr = format!("{:?}", e);
            return reply;
        }
    };

    // Check if the compiler failed
    if compiler_output.status != 0 {
        return ApiReply {
            runner: ApiRunnerInfo {
                success: false,
                stdout: String::new(),
                stderr: String::new(),
                runtime_us: 0,
            },
            compiler: ApiCompilerInfo { 
                success: false, 
                stdout: compiler_output.stdout,
                stderr: compiler_output.stderr
            },
            test_cases: vec!()
        }
    }

    // Attempt to run the code
    // TODO: Actually use challenge name
    let runner_output = match create_runner_safe(path.to_str().unwrap_or(""), request.constraints.cpu, request.constraints.ram, 1) {
        Ok(out) => out,
        Err(e) => {
            debug!("Failed to run {:?}", request.code);
            let mut reply = ApiReply::blank();
            reply.compiler.stderr = format!("{:?}", e);
            return reply;
        }
    };

    // Check if the runner failed
    if runner_output.status != 0 {
        return ApiReply {
            runner: ApiRunnerInfo {
                success: false,
                stdout: runner_output.stdout,
                stderr: runner_output.stderr,
                runtime_us: 0,
            },
            compiler: ApiCompilerInfo { 
                success: true, 
                stdout: compiler_output.stdout,
                stderr: compiler_output.stderr
            },
            test_cases: vec!()
        }
    }

    // TODO: Test cases

    ApiReply {
        runner: ApiRunnerInfo {
            success: true,
            stdout: runner_output.stdout,
            stderr: runner_output.stderr,
            runtime_us: 0, // TODO: This
        },
        compiler: ApiCompilerInfo { 
            success: true, 
            stdout: compiler_output.stdout,
            stderr: compiler_output.stderr
        },
        test_cases: vec!()
    }
}

/// Returns a json guaranteed to contain all necessary fields if the request
/// is valid, otherwise returns an error
async fn validate_request(req: Request<Body>) -> Result<ApiRequest, anyhow::Error> {
    let bytes = to_bytes(req.into_body()).await?;
    let string = String::from_utf8(bytes.to_vec())
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    let json: ApiRequest = serde_json::from_str(&string)?;

    Ok(json)
}

pub async fn post_submit_endpoint(req: Request<Body>) -> Response<Body> {
    let default_reply = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap_or_else(|_| Response::new(Body::empty()));

    match validate_request(req).await {
        Ok(json) => {
            let reply = process_reply(json);
            let body = match serde_json::to_string(&reply) {
                Ok(string) => string,
                Err(e) => {
                    // this should never happen
                    warn!("Failed to parse submit request {:#?}, {:?}", reply, e);
                    "null".to_string()
                }
            };

            Response::builder()
                        .status(StatusCode::OK)
                        .body(body.into())
                        .unwrap_or(default_reply)
        },
        Err(e) => {
            warn!("Received a bad /submit request, error: {:?}", e);
            Response::builder()
                     .status(StatusCode::BAD_REQUEST)
                     .body(Body::from(format!("{:?}", e)))
                     .unwrap_or(default_reply)
        }
    }
}