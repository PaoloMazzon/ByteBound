use std::process::{Command};
use crate::client_workspace::ClientWorkspace;
use crate::output::CommandOutput;

/// Compiles a c file in a client workspace to the file "a.out" in that same workspace
/// (ie, use client.realpath("a.out") to get the compiled file)
pub fn compile_c_file(client: &ClientWorkspace, c_file: &str) -> Result<CommandOutput, anyhow::Error> {
    let status = Command::new("gcc")
        .args(&["-static", "-o", client.realpath("a.out").as_str(), client.realpath(c_file).as_str()])
        .output()?;

    let stderr = String::from_utf8_lossy(status.stderr.as_slice());
    let stdout = String::from_utf8_lossy(status.stdout.as_slice());
    let status = status.status.code().unwrap_or(1);

    Ok(CommandOutput { status, stderr: String::from(stderr), stdout: String::from(stdout) })
}
