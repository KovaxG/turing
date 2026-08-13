#[derive(Debug, Clone)]
pub enum Operation {
  Right,
  Left,
  Erase,
  Print(char),
  Noop
}

#[derive(Debug, Clone)]
pub enum SymbolMatcher {
  Char(char),
  Everything,
  None,
  Any,
  AnyIn(Vec<char>)
}

impl SymbolMatcher {
  pub fn matches(&self, symbol: char) -> bool {
    match self {
      SymbolMatcher::Everything => true,
      SymbolMatcher::None => symbol == ' ',
      SymbolMatcher::Any => symbol != ' ',
      SymbolMatcher::AnyIn(list) => list.contains(&symbol),
      SymbolMatcher::Char(c) => symbol == *c
    }
  }
}

#[derive(Debug, Clone)]
pub struct Row {
  pub m_config: String,
  pub symbol: SymbolMatcher,
  pub operations: Vec<Operation>,
  pub final_m_config: String
}

impl Row {
  pub fn new(initial: String, symbol: SymbolMatcher, operations: Vec<Operation>, final_m_config: String) -> Row {
    Row { m_config : initial, symbol, operations, final_m_config }
  }
}
