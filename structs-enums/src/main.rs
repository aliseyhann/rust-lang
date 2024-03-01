use std::{io, panic};

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main(){
    println!("Enter the first number: ");
    let mut inp1 = String::new();
    io::stdin().read_line(&mut inp1).expect("Failed to read line");
    let num1: f64 = inp1.trim().parse().expect("Please type a valid number!");

    println!("Enter the operation (Add, Subtract, Multiply, Divide): ");
    inp1.clear();
    io::stdin().read_line(&mut inp1).expect("Failed to read line");
    let operation_str = inp1.trim().to_string();

    println!("Enter the second number:");
    inp1.clear();
    io::stdin().read_line(&mut inp1).expect("Failed to read line.");
    let num2: f64 = inp1.trim().parse().expect("Please type a valid number!");

    let operation = match operation_str.as_str() {
        "Add" => Operation::Add(num1, num2),
        "Subtract" => Operation::Subtract(num1, num2),
        "Multiply" => Operation::Multiply(num1, num2),
        "Divide" => Operation::Divide(num1, num2),
        _ => panic!("Invalid operation"),
    };

    let result = calculate(operation);

    println!("The result is: {}", result)
}