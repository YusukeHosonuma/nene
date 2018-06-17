extern crate nene;

use nene::nene::{Config, App};

fn main() {
    let config = Config::new();
    App::new(config).run();
}
