use crate::error::{Error, Result};
use structopt::StructOpt;

mod app;
mod error;
mod opt;

use app::{init, App};
use opt::Opt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    // println!("{opt:#?}");

    // let mut _launcher = init();
    // launcher.run()

    Ok(())
}
