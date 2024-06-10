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
  Process(ProcessArgs),

  /// Generate documentation
  Doc(DocArgs)
}

#[derive(clap::Args, Debug, Clone)]
pub struct ProcessArgs
{
  /// Input file or directory
  pub input: String,

  /// Additional include paths
  #[arg(short='I', long)] pub include_flags: Option<Vec<String>>,

  /// Ignore tests
  #[arg(long)] pub ignore_tests: bool
}

#[derive(clap::Args, Debug, Clone)]
pub struct DocArgs
{
  /// Input build directory with compile-commands.json
  pub input: String,

  /// Additional include paths
  #[arg(short='I', long)] pub include_flags: Option<Vec<String>>,

  /// Ignore tests
  #[arg(long)] pub ignore_tests: bool,

  /// Output format. Can be `markdown` or `m.css`
  #[arg(short, long, default_value_t = String::from("m.css"))] pub format: String,

  /// Output directory
  #[arg(long)] pub output: Option<String>
}

impl From<&DocArgs> for ProcessArgs
{
  fn from(args: &DocArgs) -> Self
  {
    Self
    {
      input: args.input.clone(),
      include_flags: args.include_flags.clone(),
      ignore_tests: args.ignore_tests
    }
  }
}