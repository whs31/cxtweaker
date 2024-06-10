use colored::Colorize;
use crate::core::names::{NAME, VERSION};

pub fn print_version_and_exit() -> !
{
  //println!("{}", ASCII_ART.yellow().bold());
  println!("{} - {} {} {} version {}",
           NAME.to_string().bright_yellow().bold(),
           "a".to_string().bold(),
           "c/c++".to_string().bold().blue(),
           "package manager!".to_string().bold(),
           VERSION.to_string().bold()
  );

  println!();
  println!("built from branch: {}", option_env!("GIT_BRANCH").unwrap_or("unknown").bold().magenta());
  println!("commit: {}", option_env!("GIT_COMMIT").unwrap_or("unknown").bold().magenta());
  println!("dirty: {}", option_env!("GIT_DIRTY").unwrap_or("unknown").bold().red());
  println!("build timestamp: {}", option_env!("SOURCE_TIMESTAMP").unwrap_or("unknown").green().bold().black());
  println!("written in rust with love");
  println!("copyright {}", "whs31 © 2024".blue().bold());
  std::process::exit(0);
}

pub fn print_error_and_exit(err: anyhow::Error) -> !
{
  eprintln!("{}: {}", "fatal error".to_string().bold().red(), err);
  std::process::exit(1);
}