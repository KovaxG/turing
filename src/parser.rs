use std::{fs, println};

use super::types::{Row, Operation};

#[derive(Debug)]
// TODO: can this be a string slice?
pub struct ParseError(String);

pub fn read_file(path: &str) -> Result<Vec<Row>, ParseError> {
  println!("Reading file: {path}");
  let contents = fs::read_to_string(path).map_err(|e| ParseError(format!("IO error: {e}")))?;
  contents
    .lines()
    .map(|l| l.to_string())
    .filter(|l| !l.is_empty())
    .filter(|l| !l.starts_with("#"))
    .map(|l| parse_line(l))
    .collect()
}

fn parse_line(line: String) -> Result<Row, ParseError> {
  let tokens: Vec<&str> = line.split('|').map(|l| l.trim()).collect();
  match tokens[..] {
    [a, b, c, d] => { 
      let commands = parse_commands(c);
      commands.map(|cs| Row::new(a.to_string(), parse_character(b), cs, d.to_string()))
    }
    _ => Err(ParseError(format!("Invalid line: '{line}'")))
  }
}

fn parse_character(character: &str) -> char {
  match character {
    "None" => ' ',
    other => other.chars().next().unwrap()
  }
}

fn parse_commands(commands: &str) -> Result<Vec<Operation>, ParseError> {
  commands.split(',').map(|c| parse_command(c)).collect()
}

fn parse_command(command: &str) -> Result<Operation, ParseError>  {
  match command {
    "E" => Ok(Operation::Erase),
    "R" => Ok(Operation::Right),
    "L" => Ok(Operation::Left),
    print => {
      if print.starts_with("P") {
        print.chars().nth(1).ok_or(ParseError(format!("Print operation incomplete: {print}"))).map(Operation::Print)
      } else {
        Err(ParseError(format!("Invalid operator: {print}")))
      }
    }
  }
}
