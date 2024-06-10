use std::path::Path;
use colored::Colorize;
use crate::core::args::ProcessArgs;
use crate::parser::opts::{CompileOption, CompileOptionFlags, CompileOptions};

pub struct Parser
{
  clang: Box<clang::Clang>,
  opts: CompileOptions
}

impl Parser
{
  pub fn new(args: &crate::args::ProcessArgs, verbose: bool) -> anyhow::Result<Self>
  {
    let clang = match clang::Clang::new() {
      Ok(c) => Box::new(c),
      Err(e) => return Err(anyhow::anyhow!("failed to initialize clang: {}", e)),
    };
    let opts = CompileOptions::from_path(Path::new(args.input.as_str()))?;
    if verbose {
      opts.pretty_print();
    }
    Ok(Parser { clang, opts })
  }

  pub fn parse(&self, args: &ProcessArgs) -> anyhow::Result<()>
  {
    let pb = indicatif::ProgressBar::new(self.opts.options.len() as u64);
    pb.set_style(indicatif::ProgressStyle::default_bar());
    pb.set_draw_target(indicatif::ProgressDrawTarget::stdout_with_hz(30));
    for opt in &self.opts.options {
      self.parse_entry(opt, args)?;
      pb.inc(1);
      pb.set_message(format!("⌛ processing {}", opt.source.file_name().unwrap().to_os_string().into_string().unwrap().bold().magenta()));
    }
    pb.finish_with_message("☑️ processing completed!");
    Ok(())
  }

  fn parse_entry(&self, opt: &CompileOption, args: &ProcessArgs) -> anyhow::Result<()>
  {
    anyhow::ensure!(opt.source.exists(), "file not found: {}", opt.source.as_path().display());
    anyhow::ensure!(opt.source.is_file(), "not a file: {}", opt.source.as_path().display());

    let index = clang::Index::new(&self.clang, false, true);
    let mut compiler_flags = opt.as_argument_array(CompileOptionFlags::REQUIRED_FOR_INDEXING);
    if args.include_flags.is_some() {
      let inc_flags = args.include_flags.clone().unwrap();
      for flag in inc_flags {
        compiler_flags.push("-isystem".to_string());
        compiler_flags.push(flag);
      }
    }
    let tu = index
      .parser(opt.source.as_path())
      .arguments(&compiler_flags)
      .parse()?;
    Ok(())
  }

  pub fn parse_file(&self, filename: &Path) -> anyhow::Result<()>
  {
    // anyhow::ensure!(filename.exists(), "file not found: {}", filename.display());
    // anyhow::ensure!(filename.is_file(), "not a file: {}", filename.display());
    //
    //let index = clang::Index::new(&self.clang, false, true);
    // let tu = index
    //   .parser(filename)
    //   .arguments(&["-x", "c++", "-std=c++20",
    //     "-I", "D:/dev/my/floppy/include",
    //     "-I", "C:/Users/User/.conan2/p/b/gtest0a588d0e1e330/p/include",
    //     "-I", "C:/Users/User/.conan2/p/fmtcdb79a57b9013/p/include",
    //     "-I", "C:/msys64/mingw64/lib/clang/18/include",
    //     "-I", "C:/Users/User/.conan2/p/b/winap9939095afc6a5/p/include",
    //     "-D", "CMAKE_PROJECT_VERSION_MAJOR=1",
    //     "-D", "CMAKE_PROJECT_VERSION_MINOR=1",
    //     "-D", "CMAKE_PROJECT_VERSION_PATCH=3",
    //     "-D", "CMAKE_TARGET_NAME=floppy",
    //     "-D", "FLOPPY_LIBRARY=1",
    //     "-D", "FMT_SHARED"
    //   ])
    //   .parse()?;
    //
    // let namespaces = tu
    //   .get_entity()
    //   .get_children()
    //   .into_iter()
    //   .filter(|ent| ent.get_kind() == clang::EntityKind::Namespace)
    //   .collect::<Vec<_>>();
    // let fns = namespaces
    //   .into_iter()
    //   .flat_map(|ns| ns.get_children())
    //   .filter(|ent| ent.get_kind() == clang::EntityKind::FunctionDecl)
    //   .collect::<Vec<_>>();
    // for fn_ in fns {
    //   match fn_.get_name() {
    //     Some(name) => println!("{}", name),
    //     None => println!("<unnamed>"),
    //   }
    // }
    Ok(())
  }
}