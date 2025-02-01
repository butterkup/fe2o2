use super::Config;
use super::USAGE;

/// Runtime variables and resources are held here
/// including but not limited to config.
pub struct Runtime {
  config: Config,
}

impl Runtime {
  /// Create a runtime from a config instance
  pub fn new(config: Config) -> Self { Self { config } }

  /// Run the application, if an error occurs,
  /// return it to the caller for handling.
  pub fn try_run(&self) -> Result<(), std::io::Error> {
    let text = std::fs::read_to_string(&self.config.file_path)?;
    crate::core::show_found(&self.config, text.lines());
    Ok(())
  }
  /// Run the application and `panic!()` if an error occurs.
  /// # Panics
  /// - If `Config::file_path` is not read successfully.
  pub fn run(&self) {
    self.try_run().unwrap_or_else(|err| {
      eprintln!("{USAGE}\n\nRunning program failed: {err}");
      std::process::exit(2);
    })
  }
}
