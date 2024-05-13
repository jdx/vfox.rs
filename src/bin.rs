#[macro_use]
extern crate log;

use std::process::exit;

mod cli;

#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(err) = cli::run().await {
        error!("{err}");
        exit(1);
    }
}
