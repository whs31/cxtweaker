use colored::Colorize;
use crate::pb_print;

pub fn ast_dump(entity: &clang::Entity) -> bool
{
  pb_print!("  [{:^24}] {:<50} in file <{}>",
    format!("{:?}", entity.get_kind()).bold(),
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
  false
}