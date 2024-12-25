pub const USAGE: &str = "\
USAGE:
  minigrep <query> <file>";

mod config;
mod runtime;
mod core;

pub use config::Config;
pub use runtime::Runtime;
