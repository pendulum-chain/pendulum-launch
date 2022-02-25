use crate::{error::Result, util};
use std::{fs, process};

/// Export genesis data to an `outdir` if provided or to the project root
pub fn export_genesis(bin: String, chain: String, name: String, outdir: String) -> Result<()> {
    // Generates genesis data, given a name
    let generate = |suffix: &str| -> Result<()> {
        let cmd = format!("export-genesis-{}", suffix);
        let output = process::Command::new(&bin)
            .args([&cmd, "--chain", &chain])
            .output()?;

        util::ensure_success(&output)?;

        let data = String::from_utf8(output.stdout)?;
        let out_file = format!("{}/{}-{}", outdir, name, suffix);
        fs::write(out_file, data)?;

        Ok(())
    };

    // Generate genesis-wasm and genesis-state
    ["wasm", "state"]
        .into_iter()
        .try_for_each(|suffix| generate(suffix))
}
