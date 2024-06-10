use colored::Colorize;
use crate::pb_print;

const ALLOWED_KINDS: [clang::EntityKind; 3] = [clang::EntityKind::FunctionTemplate, clang::EntityKind::FunctionDecl, clang::EntityKind::Method];

pub fn fn_dump(entity: &clang::Entity) -> bool
{
  if !ALLOWED_KINDS.contains(&entity.get_kind()) { return false; }

  pb_print!("  [{:^10}] {:<30} in file <{:<25}>",
    format!("{:^8}", match entity.get_kind() {
      clang::EntityKind::FunctionTemplate => "template".to_string().bright_magenta(),
      clang::EntityKind::FunctionDecl => "function".to_string().bright_blue(),
      clang::EntityKind::Method => "method".to_string().bright_cyan(),
      _ => "unknown".to_string().bright_red()
    }).bold(),
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
  true
}