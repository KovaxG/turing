mod parser;
mod turing;
mod types;

use std::{env, println};

use parser::read_file;

#[derive(Debug)]
struct Path(String);

#[derive(Debug)]
enum Setup {
  Example(Path),
  Default(Path)
}

fn mk_setup(flags: Vec<&str>, path: &str) -> Setup {
  if flags.contains(&"-e") {
    Setup::Example(Path(path.to_string()))
  } else {
    Setup::Default(Path(path.to_string()))
  }
}

fn run_setup(setup: Setup) {
  match setup {
    Setup::Example(Path(example)) => {
      let actual_path = format!("examples/{example}");
      let table = read_file(&actual_path).unwrap();
      let data = turing::run(table);
      println!("{data:?}")
    }
    Setup::Default(Path(path_str)) => {
      let table = read_file(&path_str).unwrap();
      let data = turing::run(table);
      println!("{data:?}")
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();

  let flags: Vec<&str> = args
    .iter()
    .map(String::as_str)
    .filter(|s| s.starts_with("-"))
    .collect();

  let path: &str = args
    .iter()
    .filter(|s| !s.starts_with("-"))
    .next()
    .expect("Please provide a path!");

  let setup = mk_setup(flags, path);
  run_setup(setup);
}
