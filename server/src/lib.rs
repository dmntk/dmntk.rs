#[cfg(feature = "tck")]
#[macro_use]
extern crate dmntk_macros;

mod data;
mod server;

#[cfg(feature = "tck")]
mod tck;

pub use server::start_server;
