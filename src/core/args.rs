#[derive(clap::Parser, Debug, Clone)]
#[command(name = "cxt", bin_name = "cxt")]
#[command(about = "custom preprocessor for c++", long_about = None)]
#[command(color = clap::ColorChoice::Auto)]
pub struct Args
{
  /// Execute one of major subcommands
  #[command(subcommand)] pub command: Option<Command>,

  /// Print version and exit
  #[arg(short, long)] pub version: bool
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command
{
  /// Process file or directory
  Process(ProcessArgs)
}

#[derive(clap::Args, Debug, Clone)]
pub struct ProcessArgs
{
  /// Input file or directory
  pub input: String
}