use std::{path::Path, process::Command};

pub fn compile_to_executable(ll_file_path: &Path, output_file: &Path) -> Result<(), String> {
    let status = Command::new("clang")
        .arg(ll_file_path)
        .arg("-o")
        .arg(output_file)
        .status();

    match status {
        Ok(exit_code) => {
            if !exit_code.success() {
                return Err(format!("Failed to compile, exit code: {}", exit_code));
            }
        }
        Err(e) => {
            return Err(format!("Failed to run clang: {}", e));
        }
    }

    Ok(())
}
