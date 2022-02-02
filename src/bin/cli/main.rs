use lib_pendulum_launch::{Config, Launcher};
#[allow(dead_code, unused)]
use structopt::StructOpt;

mod app;
mod error;
mod opt;
mod util;

use app::App;
use error::{Error, Result};
use opt::Options;
pub(crate) use util::locate_project_root;

fn main() -> Result<()> {
    // println!("{:?}", locate_project_root()?.into_os_string());
    // Ok(())
    let mut app = App::from_args();
    app.run()

    // let config = deserialize_config()?;
    // let mut launcher = Launcher::from(config);
    // launcher.run()?;

    // Ok(())
}

fn deserialize_config() -> Result<Config> {
    let path = std::path::PathBuf::from("./launch.json");
    let raw = std::fs::read(path)?;
    let config: Config = match serde_json::from_slice(&raw) {
        Ok(config) => config,
        Err(err) => {
            return Err(Error::Lib(lib_pendulum_launch::error::Error::Serde(
                lib_pendulum_launch::error::SerdeError::Deserialize(err.to_string()),
            )));
        }
    };

    // println!("{config:#?}");

    Ok(config)
}
