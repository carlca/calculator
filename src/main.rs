extern crate termion;

use std::io;
use std::io::Write;
use termion::input::TermRead;
use termion::event::Key;

enum Number {
  Int(i32),
  Float(f64),
}

fn main() {
  loop {
    calculate();
    let msg = "press Q or q to quit or any other key to continue...";
    println!("{}", msg);
    let key = read_single_key();
    match key {
      Some('q') | Some('Q') => break,
      _ => (),
    }
  }
}

fn calculate() {
  let operand1 = get_operand("Operand 1: ");
  let operand2 = get_operand("Operand 2: ");
  let operator = enter_operator("Operator (+,-,*,/): ");
  match operator {
    Some('+') => println!("Result is: {}", operand1 + operand2),
    Some('-') => println!("Result is: {}", operand1 - operand2),
    Some('*') => println!("Result is: {}", operand1 * operand2),
    Some('/') => {
      let result = if operand2 == 0.0 {
        "NAN".to_string()
      } else {
        (operand1 / operand2).to_string()
      };
      println!("Result is: {}", result)
    }
    _ => println!("Incorrect operator selected!"),
  };
}

fn get_operand(prompt: &str) -> f64 {
  match enter_operand(prompt) {
    Number::Int(i) => i as f64,
    Number::Float(f) => f,
  }
}

fn enter_operand(prompt: &str) -> Number {
  print!("{}", prompt);
  io::stdout().flush().unwrap();
  let mut operand = String::new();
  io::stdin()
    .read_line(&mut operand)
    .expect("Failed to read_line!");
  let trimmed = operand.trim();
  match trimmed.parse::<i32>() {
    Ok(i) => Number::Int(i),
    Err(_) => match trimmed.parse::<f64>() {
      Ok(f) => Number::Float(f),
      Err(_) => panic!("Please enter a number!"),
    },
  }
}

fn enter_operator(prompt: &str) -> Option<char> {
  print!("{}", prompt);
  io::stdout().flush().unwrap();
  match read_single_key() {
    Some('+') => { println!("Add selected."); Some('+')},
    Some('-') => { println!("Subtract selected."); Some('-')},
    Some('*') => { println!("Multiply selected."); Some('*')},
    Some('/') => { println!("Divide selected."); Some('/')},
    Some(_) => { println!("Incorrect operator selected!"); None},
    None => None,
  }
}

fn read_single_key() -> Option<char> {
  let stdin = io::stdin();
  let mut stdin_keys = stdin.lock().keys();
  match stdin_keys.next().and_then(|result| result.ok()) {
    Some(Key::Char(c)) => Some(c),
    _ => None,
  }
}