#[macro_use]
extern crate mlua;

#[cfg(test)]
#[macro_use]
extern crate insta;

// #[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;

pub use error::VFoxError;
pub use plugin::Plugin;

mod config;
mod context;
mod error;
mod hooks;
mod lua_mod;
mod metadata;
mod plugin;
mod runtime;
