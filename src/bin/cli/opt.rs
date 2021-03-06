use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(about = "Export genesis data")]
    ExportGenesis {
        #[structopt(name = "bin", parse(from_os_str), about = "Collator binary")]
        collator_bin: PathBuf,
        #[structopt(name = "chain", parse(from_os_str), about = "Collator spec")]
        collator_spec: PathBuf,
        #[structopt(short, long, about = "File prefix")]
        name: Option<String>,
        #[structopt(short, long, parse(from_os_str), about = "Alternate output directory")]
        outdir: Option<PathBuf>,
    },
    #[structopt(about = "Generate specs")]
    GenerateSpecs {
        #[structopt(name = "bin", parse(from_os_str), about = "Collator binary")]
        collator_bin: PathBuf,
        #[structopt(short, long, about = "File prefix")]
        name: Option<String>,
        #[structopt(short = "i", long, about = "Para id")]
        para_id: Option<u32>,
        #[structopt(short, long, parse(from_os_str), about = "Alternate output directory")]
        outdir: Option<PathBuf>,
    },
    #[structopt(about = "Generate docker-compose.yml")]
    GenerateDocker {
        #[structopt(short, long, parse(from_os_str), about = "Alternate output directory")]
        outdir: Option<PathBuf>,
        #[structopt(long, about = "Enable a shared docker volume for chain specs")]
        enable_volume: bool,
    },
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Options {
    #[structopt(short, long, parse(from_os_str), about = "Alternate config path")]
    pub config: Option<PathBuf>,
    #[structopt(short, long, about = "Silence output")]
    pub quiet: bool,
    #[structopt(short, long, parse(from_os_str), about = "Directoy to log node data")]
    pub log: Option<PathBuf>,
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}
