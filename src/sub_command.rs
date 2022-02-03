use crate::error::{Error, Result};
use crate::util;
use json::JsonValue;
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

/// Generate specs from a collator
pub fn generate_specs(bin: String, name: String, para_id: u32, outdir: String) -> Result<()> {
    // Generate plain
    let output = process::Command::new(&bin)
        .args(["build-spec", "--disable-default-bootnode"])
        .output()?;

    util::ensure_success(&output)?;

    let data = set_para_id(output.stdout, para_id)?;
    let out_file = format!("{}/{}-plain.json", outdir, name);

    fs::write(&out_file, data.pretty(2))?;

    // Generate raw
    let output = process::Command::new(&bin)
        .args([
            "build-spec",
            "--chain",
            &out_file,
            "--raw",
            "--disable-default-bootnode",
        ])
        .output()?;

    util::ensure_success(&output)?;

    let data = String::from_utf8(output.stdout)?;
    let out_file = format!("{}/{}-raw.json", outdir, name);
    fs::write(out_file, data)?;

    Ok(())
}

fn set_para_id(data: Vec<u8>, para_id: u32) -> Result<JsonValue> {
    let data = String::from_utf8(data)?;
    let mut serialized_data = json::parse(&data)?;

    let key = "para_id";
    match serialized_data[key] {
        JsonValue::Number(_) => {
            serialized_data[key] = JsonValue::from(para_id);
            Ok(serialized_data)
        }
        _ => Err(Error::InvalidJsonValue(key.to_string())),
    }
}
