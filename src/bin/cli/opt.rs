use crate::{Error, Result};
use lib_pendulum_launch::{Config, Launcher};
use std::{path::PathBuf, process};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(about = "Export genesis data")]
    ExportGenesis {
        #[structopt(name = "bin", parse(from_os_str), about = "Collator binary")]
        collator_bin: PathBuf,
        #[structopt(name = "chain", parse(from_os_str), about = "Collator spec")]
        collator_spec: PathBuf,
        #[structopt(short, long, parse(from_os_str), about = "Alternate output directory")]
        outdir: Option<PathBuf>,
    },
    #[structopt(about = "Generate specs")]
    GenerateSpecs {
        #[structopt(name = "bin", parse(from_os_str), about = "Collator binary")]
        collator_bin: PathBuf,
        #[structopt(short, long, parse(from_os_str), about = "Alternate output directory")]
        outdir: Option<PathBuf>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt()]
#[allow(unused)]
pub struct Options {
    #[structopt(short, long)]
    pub debug: Option<bool>,
    #[structopt(short, long, parse(from_os_str), about = "Alternate config path")]
    pub config: Option<PathBuf>,
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}
