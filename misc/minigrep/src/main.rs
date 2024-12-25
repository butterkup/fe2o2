fn main() {
  let config = minigrep::Config::build_from_args();
  let runtime = minigrep::Runtime::new(config);
  runtime.run();
}
