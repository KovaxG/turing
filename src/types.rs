#[derive(Debug, Clone)]
pub enum Operation {
  Right,
  Left,
  Erase,
  Print(char)
}

#[derive(Debug, Clone)]
pub struct Row {
  pub m_config: String,
  pub symbol: char,
  pub operations: Vec<Operation>,
  pub final_m_config: String
}

impl Row {
  pub fn new(initial: String, symbol: char, operations: Vec<Operation>, final_m_config: String) -> Row {
    Row { m_config : initial, symbol, operations, final_m_config }
  }
}
