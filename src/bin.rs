#[cfg(feature = "cli")]
#[macro_use]
extern crate log;

#[cfg(feature = "cli")]
mod cli;

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(err) = cli::run().await {
        error!("{err}");
        std::process::exit(1);
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    panic!("cli feature is not enabled");
}
