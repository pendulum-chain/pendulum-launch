use crate::{Error, Result};
use std::{path::PathBuf, process};
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(Debug)]
    enum Command {
        Launch,
        ExportGenesis,
        ExportSpecs
    }
}

#[derive(Debug, StructOpt)]
#[structopt()]
#[allow(unused)]
pub struct Opt {
    #[structopt(short, long)]
    pub debug: Option<bool>,
    #[structopt(short, long, parse(from_os_str), about = "Alternate config path")]
    pub config: Option<PathBuf>,
    #[structopt(short, long, parse(from_os_str), about = "Alternate config path")]
    pub collator_bin: Option<PathBuf>,
    #[structopt(short, long, parse(from_os_str), about = "Alternate config path")]
    pub collator_spec: Option<PathBuf>,
    #[structopt(long, about = "Collator binary for subcommands")]
    pub export_genesis: Option<bool>,
    #[structopt(
        short,
        long,
        parse(from_os_str),
        about = "Output directory for genesis data"
    )]
    pub genesis_outdir: Option<PathBuf>,
    #[structopt(long, about = "Export specs from collator binary")]
    pub export_spec: Option<bool>,
    #[structopt(short, long, parse(from_os_str), about = "Output directory for specs")]
    pub spec_outdir: Option<PathBuf>,
}

impl Opt {
    fn export_genesis(&self) -> Result<()> {
        let collator_bin = match self.genesis_outdir {
            Some(bin) => match bin.to_str() {
                Some(path) => path,
                None => return Err(Error::CollatorBin),
            },
            None => return Err(Error::CollatorBin),
        };

        let collator_spec = match self.collator_spec {
            Some(spec) => match spec.to_str() {
                Some(path) => path,
                None => return Err(Error::Genesis),
            },
            None => return Err(Error::Genesis),
        };

        let genesis_outdir = match self.genesis_outdir {
            Some(dir) => match dir.to_str() {
                Some(path) => path,
                None => return Err(Error::Genesis),
            },
            None => return Err(Error::Genesis),
        };

        process::Command::new(collator_bin)
            .args([
                "export-genesis-wasm",
                "--chain",
                collator_spec,
                ">",
                format!("{genesis_outdir}/chain-wasm").as_str(),
            ])
            .status()?;

        process::Command::new(collator_bin)
            .args([
                "export-genesis-state",
                "--chain",
                collator_spec,
                ">",
                format!("{genesis_outdir}/chain-state").as_str(),
            ])
            .status()?;

        Ok(())
    }
}
