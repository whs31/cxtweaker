use std::rc::Rc;
use clap::Parser;
use colored::Colorize;
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
      let parser = parser::Parser::new(argv, false)?;
      parser.parse(argv, |entity| {
        pb_print!("  [{}] {} in <{}>",
          format!("{:?}", entity.get_kind()).bold().yellow(),
          entity.get_name().unwrap_or("<unknown>".to_string()).bold().green(),
          match entity.get_location() {
            Some(loc) => {
              let loc = loc.get_file_location();
              let file_str = match loc.file {
                Some(file) => format!("{}", file.get_path().file_name().unwrap().to_os_string().into_string().unwrap()),
                None => "unknown".to_string()
              };
              format!("{}:{}:{}", file_str.bold().magenta(), loc.line.to_string().italic(), loc.column.to_string().italic())
            },
            None => "unknown".to_string().bold().magenta().to_string()
          }
        );
        //println!("{:?}", entity.get_name());
      })?;
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
