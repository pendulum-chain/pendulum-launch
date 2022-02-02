use crate::error::{Error, Result};
use std::path::PathBuf;
use std::process::Output;
use std::{env, process};

// Aquires the rust project root where the binary is being executed
pub fn locate_project_root() -> Result<PathBuf> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = process::Command::new(cargo)
        .arg("locate-project")
        .output()?;

    ensure_success(&output)?;

    let output = String::from_utf8(output.stdout)?;
    let parsed = json::parse(&output)?;

    // Gets project root, dropping manifest node
    let root = parsed["root"]
        .as_str()
        .ok_or_else(|| Error::ProcessFailed("no project root".to_string()))?
        .split('/')
        .skip(1)
        .fold(String::new(), |acc, entry| match entry {
            "Cargo.toml" => acc,
            _ => format!("{}/{}", acc, entry),
        });

    Ok(PathBuf::from(root))
}

pub fn ensure_success(output: &Output) -> Result<()> {
    match output.status.success() {
        true => Ok(()),
        false => {
            let msg = String::from_utf8_lossy(&output.stderr).to_string();
            Err(Error::ProcessFailed(msg))
        }
    }
}
