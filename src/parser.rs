use std::{fs, println};

use super::types::{Row, Operation};

pub fn read_file(path: &str) -> Vec<Row> {
  println!("Reading file: {path}");
  fs::read_to_string(path).unwrap().lines().map(|l| l.to_string()).map(|l| parse_line(l).unwrap()).collect()
}

fn parse_line(line: String) -> Option<Row> {
  let tokens: Vec<&str> = line.split(' ').collect();
  match tokens[..] {
    [a, b, c, d] => Some(Row::new(a.to_string(), parse_character(b), parse_commands(c), d.to_string())),
    _ => None
  }
}

fn parse_character(character: &str) -> char {
  match character {
    "None" => ' ',
    other => other.chars().next().unwrap()
  }
}

fn parse_commands(commands: &str) -> Vec<Operation> {
  commands.split(',').map(|c| parse_command(c).unwrap()).collect()
}

fn parse_command(command: &str) -> Option<Operation> {
  match command {
    "E" => Some(Operation::Erase),
    "R" => Some(Operation::Right),
    "L" => Some(Operation::Left),
    print => {
      if print.starts_with("P") {
        Some(Operation::Print(print.chars().nth(1).unwrap()))
      } else {
        None
      }
    }
  }
}
