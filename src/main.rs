use std::path::Path;
use walkdir::WalkDir;

mod structure;
use structure::diagnostic::StructureDiagnostic;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    eprintln!("Missing dir name");
    std::process::exit(1);
  }

  let dir_name = &args[1];

  let structure_rules = structure::rules::get_all_rules();
  let mut structure_diagnostics: Vec<StructureDiagnostic> = Vec::new();

  for entry in WalkDir::new(dir_name) {
    if let Ok(entry) = entry {
      let path = entry.path();
      let root = path.parent().unwrap() == Path::new(dir_name);

      for structure_rule in &structure_rules {
        if let Some(diagnostic) = structure_rule.check_file(path, root) {
          structure_diagnostics.push(diagnostic);
        }
      }
    }
  }

  if !structure_diagnostics.is_empty() {
    for d in structure_diagnostics.iter() {
      eprintln!("{}", d.to_pretty_string());
      eprintln!("Found {} structure problems", structure_diagnostics.len());
    }
  }
}
