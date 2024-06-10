use std::path::Path;
use std::time::Duration;
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
  pub fn new(args: &ProcessArgs, verbose: bool) -> anyhow::Result<Self>
  {
    let clang = match clang::Clang::new() {
      Ok(c) => Box::new(c),
      Err(e) => return Err(anyhow::anyhow!("failed to initialize clang: {}", e)),
    };
    let mut opts = CompileOptions::from_path(Path::new(args.input.as_str()))?;
    if args.ignore_tests {
      let len = opts.options.len();
      opts.options = opts
        .options
        .into_iter()
        .filter(|opt| !opt.source.to_str().unwrap().contains("test"))
        .collect();
      println!("  ☑️ discarded {} test files ({} left)",
        (len - opts.options.len()).to_string().bold().yellow(),
        opts.options.len().to_string().bold().bright_blue()
      );
    }
    if verbose {
      opts.pretty_print();
    }
    Ok(Parser { clang, opts })
  }

  pub fn parse(&self, args: &ProcessArgs) -> anyhow::Result<()>
  {
    let pb = indicatif::ProgressBar::new(self.opts.options.len() as u64)
      .with_message("⌛ processing code")
      .with_style(
        indicatif::ProgressStyle::with_template("{spinner:.cyan} {wide_msg} {human_pos:2}/{human_len:2} ({percent:3}%) [{bar:40.yellow/yellow}] [{elapsed_precise}]")
          .unwrap()
          .progress_chars("█▒░")
      );
    pb.set_draw_target(indicatif::ProgressDrawTarget::stdout_with_hz(30));
    pb.enable_steady_tick(Duration::from_millis(100));
    for opt in &self.opts.options {
      self.parse_entry(opt, args)?;
      pb.inc(1);
      pb.set_message(format!("⌛ processing {}", opt.source.file_name().unwrap().to_os_string().into_string().unwrap().bold().bright_magenta()));
    }
    pb.finish_with_message(format!("☑️ {}", String::from("processing completed!").bold().green()));
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