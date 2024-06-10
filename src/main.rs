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

  let parser = parser::Parser::new()?;
  match &args.command {
    Some(args::Command::Process(argv)) => {
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
  try_main()
    .map_err(|e| {
      core::cli::print_error_and_exit(e);
    }).unwrap();
}
