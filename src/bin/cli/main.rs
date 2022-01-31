use lib_pendulum_launch::{Config, Launcher};
#[allow(dead_code, unused)]
use structopt::StructOpt;

mod app;
mod error;
mod opt;

use app::App;
use error::{Error, Result};
use opt::Opt;

fn main() -> Result<()> {
    // let opt = Opt::from_args();
    // let mut app = App::try_from(opt)?;
    // app.run()

    let config = deserialize_config()?;
    let mut launcher = Launcher::from(config);
    launcher.run()?;

    Ok(())
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
