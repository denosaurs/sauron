extern crate dlint;
use dlint::linter::Linter;
use dlint::rules::get_all_rules;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    eprintln!("Missing file name");
    std::process::exit(1);
  }

  let file_names: Vec<String> = args[1..].to_vec();

  let mut diagnostics = vec![];

  for file_name in file_names {
    let source_code =
      std::fs::read_to_string(&file_name).expect("Failed to read file");

    let mut linter = Linter::default();

    let rules = get_all_rules();

    let file_diagnostics = linter
      .lint(file_name, source_code, rules)
      .expect("Failed to lint");

    diagnostics.extend(file_diagnostics)
  }

  if !diagnostics.is_empty() {
    for d in diagnostics.iter() {
      eprintln!(
        "error: {} ({}) at {}:{}:{}",
        d.message, d.code, d.location.filename, d.location.line, d.location.col
      );
    }
    eprintln!("Found {} problems", diagnostics.len());
  }
}
