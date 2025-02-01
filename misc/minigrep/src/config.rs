use super::USAGE;

/// Configuration used by the application
#[derive(Clone, Debug)]
pub struct Config {
  /// Query to search for in file
  pub query: String,
  /// File in which query is to be searched
  pub file_path: String,
  /// Don't take case in consideration when searching
  pub case_insentitive: bool,
}

impl Config {
  /// Try as best as possible to build the config from
  /// a set of arguments otherwise return an error
  /// for the caller to handle.  
  /// Due to current limitations, `case_insensitive`
  /// value is pulled from environment variable `MINIGREP_ICASE`
  /// whose value must be one of `'true'|'yes'|'y'|'1'` to
  /// be considered true else false.
  pub fn try_build(args: &[String]) -> Result<Self, &str> {
    if args.len() < 3 {
      Err("Not enough arguments")
    } else if args.len() > 4 {
      Err("Too many arguments")
    } else {
      Ok(Self {
        query: args[1].clone(),
        file_path: args[2].clone(),
        case_insentitive: matches!(
          std::env::var("MINIGREP_ICASE")
            .unwrap_or(String::new())
            .as_str(),
          "yes" | "y" | "true" | "1"
        ),
      })
    }
  }
  /// If `try_build` fails, panic!
  /// # Panics
  /// - If `args.len() != 3`
  pub fn build(args: &[String]) -> Self {
    Self::try_build(args).unwrap_or_else(|err| {
      eprintln!("{USAGE}\n\nParsing args failed: {err}");
      std::process::exit(1);
    })
  }
  /// Shortcut for using `env::args`, commonly used.
  pub fn build_from_args() -> Self {
    Self::build(&std::env::args().collect::<Vec<_>>())
  }
}
