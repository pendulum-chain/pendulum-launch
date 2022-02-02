// #[allow(dead_code, unused)]
mod app;
mod opt;
mod util;

use app::App;
use lib_pendulum_launch::error::{Error, Result};
use opt::Options;

fn main() -> Result<()> {
    let mut app = App::from_args();
    app.run()
}
