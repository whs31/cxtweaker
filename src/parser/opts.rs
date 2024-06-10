use std::path::{Path, PathBuf};
use colored::Colorize;
use crate::parser::json::CMakeCompileCommands;

#[derive(Debug, Clone)]
pub struct CompileOptions
{
  pub options: Vec<CompileOption>
}

#[derive(Debug, Clone)]
pub struct CompileOption
{
  pub pwd: PathBuf,
  pub definitions: Vec<(String, String)>,
  pub includes: Vec<PathBuf>,
  pub includes_system: Vec<PathBuf>,
  pub standard: String,
  pub warnings: Vec<String>,
  pub warnings_as_errors: bool,
  pub source: PathBuf,
  pub output: PathBuf
}

impl Default for CompileOptions { fn default() -> Self { CompileOptions { options: vec![] } } }
impl Default for CompileOption {
  fn default() -> Self {
    CompileOption {
      pwd: PathBuf::new(),
      definitions: vec![],
      includes: vec![],
      includes_system: vec![],
      standard: String::from("c++20"),
      warnings: vec![],
      warnings_as_errors: false,
      source: PathBuf::new(),
      output: PathBuf::new()
    }
  }
}

impl CompileOptions
{
  pub fn from_path(path: &Path) -> anyhow::Result<Self> {
    match path.is_dir() {
      true => Self::from_dir(path),
      false => Self::from_file(path),
    }
  }

  fn from_dir(dir: &Path) -> anyhow::Result<Self> {
    anyhow::ensure!(dir.exists(), "directory not found: {}", dir.display());
    anyhow::ensure!(dir.is_dir(), "not a directory: {}", dir.display());

    Self::from_file(dir.join("compile_commands.json").as_path())
  }

  fn from_file(path: &Path) -> anyhow::Result<Self>
  {
    anyhow::ensure!(path.exists(), "file not found: {}", path.display());
    anyhow::ensure!(path.is_file(), "not a file: {}", path.display());

    println!("☑️ parsing build options: {}", path.display().to_string().bold().cyan());
    let json = serde_json::from_str::<CMakeCompileCommands>(&std::fs::read_to_string(path)?)?;
    for command in json.commands {
      println!("{:?}", command);
    }

    Ok(Self::default())
  }
}