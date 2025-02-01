//! # Minigrep
//! Small grep like utility that is fairly useless
//! given the capabilities for grep, the real one.

/// Explains how to use the application
/// when reporting errors
/// ```
/// assert_eq!(4 + 6, 10);
/// ```
pub const USAGE: &str = "\
USAGE:
  minigrep <query> <file>";

mod config;
mod runtime;
mod core;

pub use config::Config;
pub use runtime::Runtime;
