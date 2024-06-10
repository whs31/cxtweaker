use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CMakeCompileCommands
{
  pub commands: Vec<CMakeCompileCommand>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CMakeCompileCommand
{
  pub directory: PathBuf,
  pub command: String,
  pub file: PathBuf,
  pub output: PathBuf
}

