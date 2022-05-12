use crate::{opt::Command, util::deserialize_config, Options};
use lib_pendulum_launch::{
    sub_command, util, Launcher, {Error, Result},
};
use std::path::PathBuf;
use structopt::StructOpt;

pub struct App(Options);

impl App {
    pub fn new(options: Options) -> Self {
        Self(options)
    }

    pub fn from_args() -> Self {
        Self::new(Options::from_args())
    }

    pub fn run(&mut self) -> Result<()> {
        match &self.0.cmd {
            Some(cmd) => match cmd {
                Command::ExportGenesis {
                    collator_bin,
                    collator_spec,
                    name,
                    outdir,
                } => self.export_genesis(
                    collator_bin.to_owned(),
                    collator_spec.to_owned(),
                    name.to_owned(),
                    outdir.to_owned(),
                )?,
                Command::GenerateSpecs {
                    collator_bin,
                    name,
                    para_id,
                    outdir,
                } => self.generate_specs(
                    collator_bin.to_owned(),
                    name.to_owned(),
                    para_id.to_owned(),
                    outdir.to_owned(),
                )?,
                Command::GenerateDocker {
                    outdir,
                    enable_volume,
                } => self.generate_docker(outdir.to_owned(), *enable_volume)?,
            },
            None => self.launch()?,
        };

        Ok(())
    }

    /// Launche parachain and idle until the program receives a `SIGINT`
    fn launch(&mut self) -> Result<()> {
        let (quiet, log) = (self.0.quiet, self.0.log.to_owned());

        if quiet && log.is_some() {
            return Err(Error::ProcessFailed(
                "Cannot use `--quiet` and `--log <DIR>` together".to_string(),
            ));
        }

        let mut config = deserialize_config(&self.0.config)?;
        config.ensure_unique_ports()?;

        Launcher::new(&mut config, log)?.run()
    }

    /// Export genesis data to an `outdir` if provided or to the project root
    fn export_genesis(
        &self,
        bin: PathBuf,
        chain: PathBuf,
        name: Option<String>,
        outdir: Option<PathBuf>,
    ) -> Result<()> {
        let bin = util::path_to_string(&bin)?;
        let chain = util::path_to_string(&chain)?;
        let name = name.unwrap_or_else(|| "local-chain".to_string());
        let outdir = util::path_to_string(&outdir.unwrap_or(util::locate_project_root()?))?;

        sub_command::export_genesis(bin, chain, name, outdir)
    }

    /// Generate specs from a collator
    fn generate_specs(
        &self,
        bin: PathBuf,
        name: Option<String>,
        para_id: Option<u32>,
        outdir: Option<PathBuf>,
    ) -> Result<()> {
        let bin = util::path_to_string(&bin)?;
        let name = name.unwrap_or_else(|| "local-chain".to_string());
        let para_id = para_id.unwrap_or(2000);
        let outdir = util::path_to_string(&outdir.unwrap_or(util::locate_project_root()?))?;

        sub_command::generate_specs(bin, name, para_id, outdir)
    }

    fn generate_docker(&self, out_dir: Option<PathBuf>, enable_volume: bool) -> Result<()> {
        let config = deserialize_config(&self.0.config)?;
        config.ensure_unique_ports()?;
        let out_dir = util::path_to_string(&out_dir.unwrap_or(util::locate_project_root()?))?;

        let command = sub_command::GenerateDocker::new(config, out_dir, enable_volume);
        command.execute()
    }
}
