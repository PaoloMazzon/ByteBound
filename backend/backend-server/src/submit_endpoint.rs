use anyhow::Context;
use hyper::{body::to_bytes, Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use spdlog::prelude::*;
use crate::{client_workspace::ClientWorkspace, compile::compile_c_file, run_container::create_runner_safe};

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
    pub challenge_index: i32,
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
}

/// Information on each ran test case
#[derive(Serialize, Debug)]
#[allow(dead_code)]
struct ApiTestCaseInfo {
    /// Runtime of this specific test case in microseconds
    runtime_us: i64,

    /// Expected value to be returned from the test case
    expected_value: String,
    
    /// Actual value returned by the user program
    actual_value: String,

    /// Input given to the user program
    input_value: String,

    /// How much memory the program used at its peak
    memory_usage_kb: usize
}

/// Represents the outgoing response, maps 1:1 to REST.md
#[derive(Serialize, Debug)]
struct ApiReply {
    /// Info from compiling the program
    pub compiler: ApiCompilerInfo,
    
    /// Info from running the program
    pub runner: ApiRunnerInfo,

    /// Runtime of the program in microseconds per test case
    runtime_us: Vec<u64>,

    /// Whether or not each test case passed
    pub test_cases: Vec<String>
}

/// Top-level function that gets an api request
async fn process_reply(request: ApiRequest) -> Result<ApiReply, anyhow::Error> {
    // Get a workspace first
    let client = ClientWorkspace::new().context("Failed to create client workspace")?;

    // Copy the proper challenge over to the workspace
    tokio::fs::copy(format!("/app/challenges/challenge_{}.json", request.challenge_index), client.realpath("challenge.json"))
        .await
        .context(format!("Failed to copy challenge code for challenge {}", request.challenge_index))?;

    // Attempt to compile the code
    let c_filename = "user_code.c";
    let main_c = format!("/app/challenges/mains/main_{}.c", request.challenge_index);
    client.write_file(c_filename, request.code.as_str())
        .context("Failed to write out client code to temporary directory.")?;

    let compiler_output = compile_c_file(&client, c_filename, main_c.as_str())
        .context("Failed to run the compiler.")?;

    // Check if the compiler failed
    if compiler_output.status != 0 {
        return Ok(ApiReply {
            runner: ApiRunnerInfo {
                success: false,
                stdout: String::new(),
                stderr: String::new(),
            },
            compiler: ApiCompilerInfo { 
                success: false, 
                stdout: compiler_output.stdout,
                stderr: compiler_output.stderr
            },
            runtime_us: vec!(),
            test_cases: vec!()
        });
    }

    // Attempt to run the code
    let runner_output = create_runner_safe(&client, request.constraints.cpu, request.constraints.ram)
        .context("Failed to run client code")?;

    // Check if the runner failed
    if runner_output.status != 0 {
        return Ok(ApiReply {
            runner: ApiRunnerInfo {
                success: false,
                stdout: runner_output.stdout,
                stderr: runner_output.stderr,
            },
            compiler: ApiCompilerInfo { 
                success: true, 
                stdout: compiler_output.stdout,
                stderr: compiler_output.stderr
            },
            runtime_us: vec!(),
            test_cases: vec!()
        });
    }

    // Grab runtimes from the runner
    let runtime_file_string = match client.read_file("runtime.txt") {
        Ok(s) => s,
        Err(e) => {
            debug!("Failed to get runtimes for request {:?}, error: {:?}", request, e);
            return Err(anyhow::anyhow!("Failed to get runtimes, error: {:?}", e));
        }
    };

    // Grab test case outputs from the runner
    let test_output_file_string = match client.read_file("test_cases_output.txt") {
        Ok(s) => s,
        Err(e) => {
            debug!("Failed to get test cases for request {:?}, error: {:?}", request, e);
            return Err(anyhow::anyhow!("Failed to get test cases, error: {:?}", e));
        }
    };
    
    let mut runtimes: Vec<u64> = runtime_file_string.split("\n")
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    let mut test_cases_outputs: Vec<String> = test_output_file_string.split("\n")
        .map(|s| s.to_string())
        .collect();
    runtimes.pop();
    test_cases_outputs.pop();

    Ok(ApiReply {
        runner: ApiRunnerInfo {
            success: true,
            stdout: runner_output.stdout,
            stderr: runner_output.stderr,
        },
        compiler: ApiCompilerInfo { 
            success: true, 
            stdout: compiler_output.stdout,
            stderr: compiler_output.stderr
        },
        runtime_us: runtimes,
        test_cases: test_cases_outputs
    })
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
            match process_reply(json).await {
                Ok(reply) => {
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
                    Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(format!("{:?}", e).into())
                                .unwrap_or(default_reply)
                }
            }
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