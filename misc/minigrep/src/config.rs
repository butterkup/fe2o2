use super::USAGE;

#[derive(Clone, Debug)]
pub struct Config {
  pub query: String,
  pub file_path: String,
  pub case_insentitive: bool,
}

impl Config {
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

  pub fn build(args: &[String]) -> Self {
    Self::try_build(args).unwrap_or_else(|err| {
      eprintln!("{USAGE}\n\nParsing args failed: {err}");
      std::process::exit(1);
    })
  }

  pub fn build_from_args() -> Self {
    Self::build(&std::env::args().collect::<Vec<_>>())
  }
}
