use std::process::Command;
use std::io;
use std::path::Path;

pub fn create_runner(binary_path: &str) -> io::Result<()> {
    let host_bin_dir = "../docker";
    //let bin_path = Path::new(binary_path);

    // Ensure the binary filename is valid
    let file_name = Path::new(binary_path)
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid binary path"))?;

     let status = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &format!("{}:/host_bin", host_bin_dir),
            "runner",
            &format!("/host_bin/{}", file_name),
        ])
        .status()?; // execute command

     if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Runner container execution failed"));
    }

    Ok(())
}
