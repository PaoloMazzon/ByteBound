use std::process::{Command};
use std::path::PathBuf;
use crate::output::CommandOutput;


pub fn compile_c_file(c_file: &str, output_file: &str) -> Result<(PathBuf, CommandOutput), anyhow::Error> {
    let status = Command::new("gcc")
        .args(&["-static", "-o", output_file, c_file])
        .output()?;

    let stderr = String::from_utf8_lossy(status.stderr.as_slice());
    let stdout = String::from_utf8_lossy(status.stdout.as_slice());
    let status = status.status.code().unwrap_or(1);

    Ok((PathBuf::from(output_file), CommandOutput { status, stderr: String::from(stderr), stdout: String::from(stdout) }))
}
