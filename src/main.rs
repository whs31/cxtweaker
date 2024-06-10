use std::path::Path;
use std::rc::Rc;
use clap::Parser;
use crate::core::args;

mod core;
mod parser;

fn try_main() -> anyhow::Result<()>
{
  let args = Rc::new(core::Args::parse());
  if args.version {
    core::cli::print_version_and_exit();
  }

  match &args.command {
    Some(args::Command::Process(argv)) => {
      let parser = parser::Parser::new(argv)?;
      parser.parse_file(Path::new(&argv.input))?;
    }
    None => {
      return Err(anyhow::anyhow!("no command specified. see --help"));
    }
  }
  Ok(())
}

fn main()
{
  std::env::set_var("RUST_BACKTRACE", "1");
  try_main()
    .map_err(|e| {
      core::cli::print_error_and_exit(e);
    }).unwrap();
}
