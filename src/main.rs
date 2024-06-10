use std::rc::Rc;
use clap::Parser;
use crate::core::args;

mod core;
mod parser;
mod algo;

fn try_main() -> anyhow::Result<()>
{
  let args = Rc::new(core::Args::parse());
  if args.version {
    core::cli::print_version_and_exit();
  }

  match &args.command {
    Some(args::Command::Process(argv)) => {
      let parser = parser::Parser::new(argv, false, None)?;
      parser.parse(argv, algo::misc::ast_dump)?;
    },
    Some(args::Command::Doc(argv)) => {
      let argv2 = args::ProcessArgs::from(argv);
      let parser = parser::Parser::new(&argv2, false, None)?;
      parser.parse(&argv2, algo::misc::ast_dump)?;
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
