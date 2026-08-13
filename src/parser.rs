use std::{fs, println};

use super::types::{Row, Operation, SymbolMatcher};

#[derive(Debug)]
pub struct ParseError(String);

pub fn read_file(path_raw: &str) -> Result<Vec<Row>, ParseError> {
  let path = fix_path(path_raw);
  println!("Reading file: {path}");
  fs::read_to_string(path)
    .map_err(|e| ParseError(format!("IO error: {e}")))?
    .lines()
    .map(|l| l.to_string())
    .filter(|l| !l.is_empty())
    .filter(|l| !l.starts_with("#"))
    .map(parse_line)
    .collect()
}

fn parse_line(line: String) -> Result<Row, ParseError> {
  let tokens: Vec<&str> = line.split('|').map(|l| l.trim()).collect();
  match tokens[..] {
    [a, b, c, d] =>
      parse_commands(c)
        .map(|cs| Row::new(a.to_string(), parse_symbol(b), cs, d.to_string())),
    _ => Err(ParseError(format!("Invalid line: '{line}'")))
  }
}

fn parse_symbol(symbol: &str) -> SymbolMatcher {
  match symbol {
    "" => SymbolMatcher::Everything,
    "None" => SymbolMatcher::None,
    text =>
      if text.starts_with("Any") {
        if text.contains("(") && text.contains(")") {
          let characters =
            text
              .split("(")
              .collect::<Vec<&str>>()
              .get(1)
              .unwrap()
              .chars()
              .filter(|c| *c != ')' && *c != ',')
              .collect::<Vec<char>>();

          SymbolMatcher::AnyIn(characters)
        } else {
          SymbolMatcher::Any
        }
      } else {
        SymbolMatcher::Char(text.chars().next().unwrap())
      }
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
    "" => Ok(Operation::Noop),
    print => {
      if print.starts_with("P") {
        print
          .chars()
          .nth(1)
          .ok_or(ParseError(format!("Print operation incomplete: {print}")))
          .map(Operation::Print)
      } else {
        Err(ParseError(format!("Invalid operator: {print}")))
      }
    }
  }
}

fn fix_path(path: &str) -> String {
  if path.ends_with(".tur") {
    path.to_string()
  } else {
    format!("{path}.tur")
  }
}
