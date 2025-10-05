use std::process::Command;
use std::path::PathBuf;


fn compile_c_file(c_file: &str, output_file: &str) -> Result<PathBuf, String> {
    let status = Command::new("gcc")
        .args(&["-o", output_file, c_file])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err(format!("Compilation failed with code {:?}", status.code()));
    }

    Ok(PathBuf::from(output_file))
}

// Testing
fn main() {
    let compiled = compile_c_file("test.c", "test").unwrap();
    println!("{}", compiled.display());
}