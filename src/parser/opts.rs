use std::path::{Path, PathBuf};
use colored::Colorize;
use crate::parser::json::{CMakeCompileCommand, CMakeCompileCommands};
// flags
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct CompileOptionFlags: u32
  {
    const INCLUDES           = 0b00000001;
    const INCLUDES_SYSTEM    = 0b00000010;
    const DEFINITIONS        = 0b00000100;
    const WARNINGS           = 0b00001000;
    const WARNINGS_AS_ERRORS = 0b00010000;
    const STANDARD           = 0b00100000;
    const SOURCE             = 0b01000000;
    const OUTPUT             = 0b10000000;

    const ALL = Self::INCLUDES.bits()
      | Self::INCLUDES_SYSTEM.bits()
      | Self::DEFINITIONS.bits()
      | Self::WARNINGS.bits()
      | Self::WARNINGS_AS_ERRORS.bits()
      | Self::STANDARD.bits()
      | Self::SOURCE.bits()
      | Self::OUTPUT.bits();

    const REQUIRED_FOR_INDEXING = Self::INCLUDES.bits()
      | Self::INCLUDES_SYSTEM.bits()
      | Self::DEFINITIONS.bits()
      | Self::STANDARD.bits();
  }
}

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

  pub fn from_string(s: &str) -> anyhow::Result<Self> {
    let json = serde_json::from_str::<CMakeCompileCommands>(s)?;
    let mut options = vec![];
    for command in json.commands {
      options.push(CompileOption::from(&command));
    }

    println!("  ☑️ successfully parsed {} build options", options.len().to_string().bold().bright_blue());
    Ok(Self { options })
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

    println!("  ☑️ parsing build options: {}", path.display().to_string().bold().cyan());
    let contents = std::fs::read_to_string(path)?;
    Self::from_string(&contents)
  }

  pub fn pretty_print(&self)
  {
    if self.options.is_empty() {
      println!("{}", "❌ no build options found".to_string().bold().red());
      return;
    }
    println!("working directory: {}", self.options[0].pwd.display().to_string().bold().cyan());
    println!("file count: {}", self.options.len().to_string().bold().bright_green());
    println!("files: [");

    for option in &self.options {
      println!("\t{}", option.source.file_name().unwrap().to_os_string().into_string().unwrap().bold().white());
    }
    println!("]\n");
    println!("first entry:");
    self.options[0].pretty_print();
    println!("\n...and {} other files", (self.options.len() - 1).to_string().bold().bright_green());
  }
}

impl From<&CMakeCompileCommand> for CompileOption
{
  fn from(that: &CMakeCompileCommand) -> Self {
    let re = regex::Regex::new(r" -D(\S*)=(\S*)").unwrap();
    let definitions = re
      .captures_iter(&that.command)
      .map(|cap| (cap[1].to_string(), cap[2].to_string()))
      .collect::<Vec<_>>();

    let re = regex::Regex::new(r" -I(\S*)").unwrap();
    let includes = re
      .captures_iter(&that.command)
      .map(|cap| PathBuf::from(cap[1].to_string()))
      .collect::<Vec<_>>();

    let re = regex::Regex::new(r" -isystem\s?(\S*)").unwrap();
    let includes_system = re
      .captures_iter(&that.command)
      .map(|cap| PathBuf::from(cap[1].to_string()))
      .collect::<Vec<_>>();

    let re = regex::Regex::new(r" -W(\S*)").unwrap();
    let mut warnings = re
      .captures_iter(&that.command)
      .map(|cap| cap[1].to_string())
      .collect::<Vec<_>>();
    let mut warnings_as_errors = false;
    if warnings.contains(&"error".to_string()) {
      warnings_as_errors = true;
      warnings.retain(|w| w != "error");
    }

    let re = regex::Regex::new(r" -std=(\S+)").unwrap();
    let standard = re
      .captures_iter(&that.command)
      .map(|cap| cap[1].to_string())
      .next()
      .unwrap_or("c++20".to_string());

    CompileOption {
      pwd: that.directory.clone(),
      definitions,
      includes,
      includes_system,
      standard,
      warnings,
      warnings_as_errors,
      source: that.file.clone(),
      output: that.output.clone()
    }
  }
}

impl CompileOption
{
  pub fn pretty_print(&self)
  {
    println!("\tsource: {}", self.source.display().to_string().bold().green());
    println!("\tc++ standard: {}", self.standard.bold().magenta());
    println!("\tdefinitions: [");
    for def in &self.definitions {
      println!("\t\t{} = {}", def.0.bold().blue(), def.1.bold().bright_magenta());
    }
    println!("\t]");
    println!("\tinclude paths: [");
    for inc in &self.includes {
      println!("\t\t{}", inc.display().to_string().bold().white());
    }
    println!("\t]");
    println!("\tsystem include paths: [");
    for inc in &self.includes_system {
      println!("\t\t{}", inc.display().to_string().dimmed().white());
    }
    println!("\t]");
    println!("\twarnings: [{}]", self.warnings.join(", ").bold().yellow());
    println!("\twarnings_as_errors: {}", self.warnings_as_errors.to_string().bold().cyan());
    println!("\toutput: {}", self.output.display().to_string().dimmed().cyan());
    println!("\tpwd: {}", self.pwd.display().to_string().dimmed().blue());
  }

  pub fn as_argument_array(&self, flags: CompileOptionFlags) -> Vec<String>
  {
    let mut args = vec!["-x".to_string(), "c++".to_string(), "-g".to_string()];
    if flags.contains(CompileOptionFlags::STANDARD) {
      args.push(format!("-std={}", self.standard));
    }
    if flags.contains(CompileOptionFlags::WARNINGS) && !self.warnings.is_empty() {
      for warn in &self.warnings {
        args.push("-W".to_string());
        args.push(warn.clone());
      }
    }
    if flags.contains(CompileOptionFlags::WARNINGS_AS_ERRORS) && self.warnings_as_errors {
      args.push("-W".to_string());
      args.push("error".to_string());
    }
    if flags.contains(CompileOptionFlags::DEFINITIONS) && !self.definitions.is_empty() {
      for def in &self.definitions {
        args.push("-D".to_string());
        args.push(format!("{}={}", def.0, def.1));
      }
    }
    if flags.contains(CompileOptionFlags::INCLUDES) && !self.includes.is_empty() {
      for inc in &self.includes {
        args.push("-I".to_string());
        args.push(format!("{}", inc.display().to_string()));
      }
    }
    if flags.contains(CompileOptionFlags::INCLUDES_SYSTEM) && !self.includes_system.is_empty() {
      for inc in &self.includes_system {
        args.push("-isystem".to_string());
        args.push(format!("{}", inc.display().to_string()));
      }
    }
    args
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_parser()
  {
    // include file from test_data/example1
    let data_str = include_str!("test_data/example1.json");
    let got_vec = CompileOptions::from_string(data_str).unwrap();

    assert_eq!(got_vec.options.len(), 7);
    let got = got_vec.options[0].clone();
    assert_eq!(got.source.display().to_string(), r#"D:\dev\my\floppy\src\detail\rtti.cc"#.to_string());
    assert_eq!(got.output.display().to_string(), r#"CMakeFiles\floppy.dir\src\detail\rtti.cc.obj"#.to_string());
    assert_eq!(got.pwd.display().to_string(), r#"D:/dev/my/floppy/build/Debug"#.to_string());
    assert_eq!(got.definitions.len(), 5);
    assert_eq!(got.definitions[0], ("CMAKE_PROJECT_VERSION_MAJOR".to_string(), "1".to_string()));
    assert_eq!(got.definitions[1], ("CMAKE_PROJECT_VERSION_MINOR".to_string(), "1".to_string()));
    assert_eq!(got.definitions[2], ("CMAKE_PROJECT_VERSION_PATCH".to_string(), "3".to_string()));
    assert_eq!(got.definitions[3], ("CMAKE_TARGET_NAME".to_string(), "floppy".to_string()));
    assert_eq!(got.definitions[4], ("FLOPPY_LIBRARY".to_string(), "1".to_string()));
    assert_eq!(got.includes.len(), 4);
    assert_eq!(got.includes[0].display().to_string(), "D:/dev/my/floppy/build/Debug".to_string());
    assert_eq!(got.includes[1].display().to_string(), "D:/dev/my/floppy".to_string());
    assert_eq!(got.includes[2].display().to_string(), "D:/dev/my/floppy/include".to_string());
    assert_eq!(got.includes[3].display().to_string(), "D:/dev/my/floppy/src/c++".to_string());
    assert_eq!(got.includes_system.len(), 2);
    assert_eq!(got.includes_system[0].display().to_string(), "C:/Users/User/.conan2/p/fmtcdb79a57b9013/p/include".to_string());
    assert_eq!(got.includes_system[1].display().to_string(), "C:/Users/User/.conan2/p/b/winap9939095afc6a5/p/include".to_string());
    assert_eq!(got.warnings.len(), 3);
    assert_eq!(got.warnings[0], "all".to_string());
    assert_eq!(got.warnings[1], "extra".to_string());
    assert_eq!(got.warnings[2], "pedantic".to_string());
    assert_eq!(got.warnings_as_errors, true);

    assert_eq!(got.as_argument_array(CompileOptionFlags::ALL), [
      "-x", "c++", "-g",
      "-std=c++20",
      "-W", "all",
      "-W", "extra",
      "-W", "pedantic",
      "-W", "error",
      "-D", "CMAKE_PROJECT_VERSION_MAJOR=1",
      "-D", "CMAKE_PROJECT_VERSION_MINOR=1",
      "-D", "CMAKE_PROJECT_VERSION_PATCH=3",
      "-D", "CMAKE_TARGET_NAME=floppy",
      "-D", "FLOPPY_LIBRARY=1",
      "-I", "D:/dev/my/floppy/build/Debug",
      "-I", "D:/dev/my/floppy",
      "-I", "D:/dev/my/floppy/include",
      "-I", "D:/dev/my/floppy/src/c++",
      "-isystem", "C:/Users/User/.conan2/p/fmtcdb79a57b9013/p/include",
      "-isystem", "C:/Users/User/.conan2/p/b/winap9939095afc6a5/p/include"
    ]);

    assert_eq!(got.as_argument_array(CompileOptionFlags::INCLUDES
      | CompileOptionFlags::INCLUDES_SYSTEM
      | CompileOptionFlags::DEFINITIONS
      | CompileOptionFlags::STANDARD
    ), [
      "-x", "c++", "-g",
      "-std=c++20",
      "-D", "CMAKE_PROJECT_VERSION_MAJOR=1",
      "-D", "CMAKE_PROJECT_VERSION_MINOR=1",
      "-D", "CMAKE_PROJECT_VERSION_PATCH=3",
      "-D", "CMAKE_TARGET_NAME=floppy",
      "-D", "FLOPPY_LIBRARY=1",
      "-I", "D:/dev/my/floppy/build/Debug",
      "-I", "D:/dev/my/floppy",
      "-I", "D:/dev/my/floppy/include",
      "-I", "D:/dev/my/floppy/src/c++",
      "-isystem", "C:/Users/User/.conan2/p/fmtcdb79a57b9013/p/include",
      "-isystem", "C:/Users/User/.conan2/p/b/winap9939095afc6a5/p/include"
    ]);

    assert_eq!(got.as_argument_array(
      CompileOptionFlags::INCLUDES
      | CompileOptionFlags::INCLUDES_SYSTEM
      | CompileOptionFlags::DEFINITIONS
      | CompileOptionFlags::STANDARD
    ), got.as_argument_array(
      CompileOptionFlags::ALL
      &! CompileOptionFlags::WARNINGS
      &! CompileOptionFlags::WARNINGS_AS_ERRORS
    ));
    assert_eq!(got.as_argument_array(
      CompileOptionFlags::INCLUDES
        | CompileOptionFlags::INCLUDES_SYSTEM
        | CompileOptionFlags::DEFINITIONS
        | CompileOptionFlags::STANDARD
    ), got.as_argument_array(CompileOptionFlags::REQUIRED_FOR_INDEXING)
    );
  }
}