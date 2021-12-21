use anyhow::Result;
use rexplode::explode;

fn print_help() {
  eprintln!(
    "{name} {version}

{description}

USAGE:
    {name} <PATTERN>

ARGS:
    <PATTERN>        Specify a pattern to generate strings

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information",
    name = env!("CARGO_BIN_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    description = env!("CARGO_PKG_DESCRIPTION"),
  )
}

fn print_version() {
  eprintln!(
    "{name} {version}",
    name = env!("CARGO_BIN_NAME"),
    version = env!("CARGO_PKG_VERSION"),
  )
}

fn print_strings(pattern: &str) -> Result<()> {
  for result in explode(pattern)? {
    println!("{}", result)
  }

  Ok(())
}

fn main() -> Result<()> {
  match std::env::args().nth(1) {
    Some(pattern) if pattern == "-h" || pattern == "--help" => print_help(),
    Some(pattern) if pattern == "-V" || pattern == "--version" => print_version(),
    Some(pattern) => print_strings(&pattern)?,
    None => print_help(),
  }

  Ok(())
}
