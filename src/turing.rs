use crate::types::{Row, Operation};

const TAPE_LENGTH: usize = 20;

pub fn run(m_table: Vec<Row>) -> Vec<char> {
  let mut pos = 0;
  let mut tape = vec![' '].repeat(TAPE_LENGTH);
  let mut config = m_table.first().unwrap().m_config.clone();
  loop {
    let tape_str: String = tape.iter().collect();
    let extra_info = format!("c: {config}, i: {pos:2} → ");
    println!("{extra_info}[{tape_str}]");

    let carret_str: String = vec![' '].repeat(pos + extra_info.chars().count() + 1).iter().collect();
    println!("{carret_str}▲\n");

    let (c, t, p) = run_step(m_table.clone(), tape.clone(), pos, config.clone());
    config = c;
    tape = t;
    pos = p;

  }
}

// returns the next m_config, the new tape and the position on the tape
fn run_step(m_table: Vec<Row>, tape: Vec<char>, tape_position: usize, current_m_config: String) -> (String, Vec<char>, usize) {
  let cur_row =
    m_table
      .into_iter()
      .find(|r| r.m_config == current_m_config && r.symbol.matches(*tape.get(tape_position).unwrap()))
      .expect("Invalid m-config!");

  let (new_tape, new_pos) =
    cur_row
      .operations
      .iter()
      .fold((tape, tape_position), |(tape, pos), op| execute_operation(tape, pos, op));

  (cur_row.final_m_config, new_tape, new_pos)
}

// TODO(Gyuri): might be good to check tape bounds
fn execute_operation(tape: Vec<char>, tape_pos: usize, operation: &Operation) -> (Vec<char>, usize) {
  match operation {
    Operation::Erase => execute_operation(tape, tape_pos, &Operation::Print(' ')),
    Operation::Right => (tape, tape_pos + 1),
    Operation::Left => (tape, tape_pos - 1),
    Operation::Print(c) => {
      let mut new_tape = tape.clone();
      new_tape[tape_pos] = *c;
      (new_tape, tape_pos)
    }
  }
}