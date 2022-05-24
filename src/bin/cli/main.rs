mod app;
mod opt;
mod util;

use app::App;
use lib_pendulum_launch::Result;
use opt::Options;

fn main() -> Result<()> {
    App::from_args()?.run()
}

fn do_nothing() {}
