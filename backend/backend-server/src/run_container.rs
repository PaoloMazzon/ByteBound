use std::{process::Command};
use crate::{client_workspace::ClientWorkspace, output::CommandOutput};

pub fn create_runner_safe(client: &ClientWorkspace, cpu_limit: i64, memory_limit: i64) -> Result<CommandOutput, anyhow::Error> {
    let normalized_cpu_limit = (cpu_limit as f64 / 1000.0) / 2.0;
    let actual_mem = memory_limit / 1024;

    let output = Command::new("timeout")
        .args([
            "20s",
            "docker",
            "run",
            "--rm",
            "-v",
            client.docker_volume_flag().as_str(),
            "runner",
            &normalized_cpu_limit.to_string(),
            &actual_mem.to_string(),
        ]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let status = output.status.code().unwrap_or(1);

    Ok(CommandOutput { status, stdout: String::from(stdout), stderr: String::from(stderr) })
}
