mod parser;
mod types;

use types::{Row, Operation};
use parser::read_file;

fn run(m_table: Vec<Row>) -> Vec<char> {
  let mut pos = 0;
  let mut tape = vec![' '].repeat(20);
  let mut config = m_table.first().unwrap().m_config.clone();
  loop {
    println!("c: {config}, i: {pos} -> {tape:?}");

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
      .find(|r| r.m_config == current_m_config && r.symbol == *tape.get(tape_position).unwrap())
      .expect("Invalid m-config!");

  let (new_tape, new_pos) =
    cur_row.operations.iter()
    .fold((tape, tape_position), |(tape, pos), op| execute_operation(tape, pos, op));

  (cur_row.final_m_config, new_tape, new_pos)
}

// TODO(Gyuri): might be good to check tape bounds
fn execute_operation(tape: Vec<char>, tape_pos: usize, operation: &Operation) -> (Vec<char>, usize) {
  match operation {
    Operation::Right => (tape, tape_pos + 1),
    Operation::Left => (tape, tape_pos - 1),
    Operation::Erase => {
      let mut new_tape = tape.clone();
      new_tape[tape_pos] = ' ';
      (new_tape, tape_pos)
    },
    Operation::Print(c) => {
      let mut new_tape = tape.clone();
      new_tape[tape_pos] = *c;
      (new_tape, tape_pos)
    }
  }
}

fn main() {
  let table = read_file("examples/simple.tur");
  let data = run(table);
  println!("{data:?}");
}
