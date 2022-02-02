#[allow(dead_code, unused)]
mod app;
mod error;
mod opt;
mod util;

use app::App;
use error::{Error, Result};
use opt::Options;

fn main() -> Result<()> {
    let mut app = App::from_args();
    app.run()
}
