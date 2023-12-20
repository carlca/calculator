use std::io;
use std::io::Read;
use std::io::Write;

enum Number {
    Int(i32),
    Float(f64),
}

fn main() {
    let operand1 = enter_operand("Operand 1: ");
    let operand1 = match operand1 {
        Number::Int(i) => i as f64,
        Number::Float(f) => f,
    };
    let operand2 = enter_operand("Operand 2: ");
    let operand2 = match operand2 {
        Number::Int(i) => i as f64,
        Number::Float(f) => f,
    };

    let mut operator = enter_operator("Operator (+,-,*,/): ");
    match operator {
        '+' => println!("Result is: {}", operand1 + operand2),
        '-' => println!("Result is: {}", operand1 - operand2),
        '*' => println!("Result is: {}", operand1 * operand2),
        '/' => {
            let result = if operand2 == 0.0 {
                "NAN".to_string()
            } else {
                (operand1 / operand2).to_string()
            };
            println!("Result is: {}", result)},
        _ => println!("Incorrect operator selected!")
    };
}

fn enter_operand(prompt: &str) -> Number {
    print!("{}", prompt);
    io::stdout().flush().unwrap();use std::io::Write;
    let mut operand = String::new();
    io::stdin().read_line(&mut operand).expect("Failed to read_line!");
    let trimmed = operand.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => Number::Int(i),
        Err(_) => match trimmed.parse::<f64>() {
            Ok(f) => Number::Float(f),
            Err(_) => panic!("Please enter a number!"),
        },
    }
}

fn enter_operator(prompt: &str) -> char {
    let mut stdin = io::stdin();
    let mut buffer = [0; 1];
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    stdin.read_exact(&mut buffer).unwrap();
    let operator = buffer[0] as u8 as char;
    match operator {
        '+' => println!("Add selected."),
        '-' => println!("Subtract selected."),
        '*' => println!("Multiply selected."),
        '/' => println!("Divide selected."),
        _ => println!("Incorrect operator selected!")
    }
    io::stdout().flush().unwrap();use std::io::Write;
    operator
}