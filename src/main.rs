extern crate nene;

use nene::nene::{App, Config};

fn main() {
    let config = Config::new();
    App::new(config).run();
}
