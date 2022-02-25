// #[allow(dead_code, unused)]
mod app;
mod opt;

use app::App;
use lib_pendulum_launch::error::Result;
use opt::Options;

fn main() -> Result<()> {
    App::from_args().run()
}
