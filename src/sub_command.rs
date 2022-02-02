use crate::error::{Error, Result};
use std::{fs, process};

/// Export genesis data to an `outdir` if provided or to the project root
pub fn export_genesis(bin: String, chain: String, outdir: String) -> Result<()> {
    // Generates genesis data, given a name
    let generate = |name: &str| -> Result<()> {
        let cmd = format!("export-genesis-{name}");
        let output = process::Command::new(&bin)
            .args([&cmd, "--chain", &chain])
            .output()?;

        if !output.status.success() {
            return Err(Error::ProcessFailed(output.stderr));
        }

        let data = String::from_utf8(output.stdout)?;
        let out_file = format!("{outdir}/chain-{name}");
        fs::write(out_file, data)?;

        Ok(())
    };

    // Generate genesis-wasm and genesis-state
    ["wasm", "state"]
        .into_iter()
        .try_for_each(|name| generate(name))
}
