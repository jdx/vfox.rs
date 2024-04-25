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

mod error;
mod plugin;
mod hooks;
mod metadata;
mod context;
mod runtime;
