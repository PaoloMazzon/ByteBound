use std::process::Command;
use anyhow::anyhow;

pub fn create_runner_safe(binary_path: &str, cpu_limit: i64, memory_limit: i64, problem_index: i32) -> Result<String, anyhow::Error> {
    let normalized_cpu_limit = (cpu_limit as f64 / 1000.0) / 2.0;
    let actual_mem = memory_limit / 1024;

    let question = format!("/questions/{}.json:/question", problem_index);
    let binary_path_volume = format!("{}:/binary", binary_path);
    let mut binding = Command::new("timeout");
    let output = binding
        .args([
            "20s",
            "docker",
            "run",
            "--rm",
            "-v",
            "/output.txt",
            "-v",
            &question,
            "-v",
            &binary_path_volume,
            "runner",
            "/binary",
            &normalized_cpu_limit.to_string(),
            &actual_mem.to_string(),
        ]).output()?;
    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    match status.success() {
        true => Ok(stdout.to_string()),
        false => Err(anyhow!("Runner container execution failed, stdout: {:?}\nstderr: {:?}", stdout, stderr))
    }
}
