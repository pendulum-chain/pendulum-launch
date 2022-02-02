use crate::{Error, Result};
use std::path::PathBuf;
use std::{env, process};

// Aquires the rust project root where the binary is being executed
pub fn locate_project_root() -> Result<PathBuf> {
    let cargo = env::var("CARGO").unwrap_or("cargo".to_owned());
    let output = process::Command::new(cargo)
        .arg("locate-project")
        .output()?;

    if !output.status.success() {
        return Err(Error::ProcessFailed(output.stderr));
    }

    let output = String::from_utf8(output.stdout)?;
    let parsed = json::parse(&output)?;
    // Gets project root, dropping manifest node
    let root = parsed["root"]
        .as_str()
        .ok_or(Error::ProcessFailed(b"no project root".to_vec()))?
        .split('/')
        .skip(1)
        .fold(String::new(), |acc, entry| match entry {
            "Cargo.toml" => acc,
            _ => format!("{acc}/{entry}"),
        });

    Ok(PathBuf::from(root))
}
