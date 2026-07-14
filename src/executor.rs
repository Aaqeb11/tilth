use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct TempTfVars {
    path: PathBuf,
}

impl TempTfVars {
    fn create(dir: &Path, vars: &HashMap<String, String>) -> Result<Self, std::io::Error> {
        let file_path = dir.join("tilth-temp.tfvars.json");
        let json_string = serde_json::to_string(vars)?;
        fs::write(&file_path, json_string)?;
        Ok(TempTfVars { path: file_path })
    }

    fn file_name(&self) -> &std::ffi::OsStr {
        // Safe to unwrap because we defined the filename exactly when creating it above.
        self.path.file_name().unwrap()
    }
}

impl Drop for TempTfVars {
    fn drop(&mut self) {
        // We use `let _ =` to intentionally ignore any errors during cleanup.
        // If it fails to delete (e.g. permission issues), we don't want to panic during a drop.
        let _ = fs::remove_file(&self.path);
    }
}

pub fn run_terraform(
    command: &str,
    path: &Path,
    variables: &HashMap<String, String>,
    extra_args: &[String],
) -> Result<(), std::io::Error> {
    
    // 1. Generate the temp tfvars file
    let temp_vars = TempTfVars::create(path, variables)?;

    println!("Running terraform {} in {}...", command, path.display());

    // 2. Build the terraform command
    let mut cmd = Command::new("terraform");
    cmd.current_dir(path)
        .arg(command)
        .arg(format!("-var-file={}", temp_vars.file_name().to_string_lossy()));

    // Add any extra arguments the user passed through (e.g., -target)
    for arg in extra_args {
        cmd.arg(arg);
    }

    // 3. Execute and wait for it to finish, inheriting stdout/stderr natively
    let status = cmd.status()?;

    if !status.success() {
        eprintln!("Terraform exited with an error.");
    }

    // `temp_vars` goes out of scope here, and the Drop trait automatically deletes the file!
    Ok(())
}
