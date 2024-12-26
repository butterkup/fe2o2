use super::Config;
use super::USAGE;

pub struct Runtime {
  config: Config,
}

impl Runtime {
  pub fn new(config: Config) -> Self { Self { config } }

  pub fn try_run(&self) -> Result<(), std::io::Error> {
    let text = std::fs::read_to_string(&self.config.file_path)?;
    crate::core::show_found(&self.config, text.lines());
    Ok(())
  }

  pub fn run(&self) {
    self.try_run().unwrap_or_else(|err| {
      eprintln!("{USAGE}\n\nRunning program failed: {err}");
      std::process::exit(2);
    })
  }
}
