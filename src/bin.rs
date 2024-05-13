pub use vfox::Vfox;
pub use error::VFoxError;
pub use plugin::Plugin;

#[macro_use]
extern crate mlua;

mod cli;
mod vfox;
mod config;
mod plugin;
mod error;
mod runtime;
mod lua_mod;
mod metadata;
mod context;

#[tokio::main]
async fn main() {
    cli::run().await;
}
