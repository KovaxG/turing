#[derive(Debug, Clone)]
pub enum Operation {
  Right,
  Left,
  Erase,
  Print(char)
}

#[derive(Debug, Clone)]
pub enum SymbolMatcher {
  Char(char),
  None,
  Any
}

impl SymbolMatcher {
  pub fn matches(&self, symbol: char) -> bool {
    match self {
      SymbolMatcher::None => symbol == ' ',
      SymbolMatcher::Any => symbol != ' ',
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
