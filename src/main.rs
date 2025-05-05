mod calculate;
mod operator;

use calculate::calculate;
use operator::Operator;
use std::io;

fn prompt_for_number(prompt: &str) -> i32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
        .trim()
        .parse::<i32>()
        .expect("Please enter a valid number")
}

fn prompt_for_char(prompt: &str) -> char {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().chars().next().expect("Please enter a valid character")
}

fn main() {
    println!("Calculator App");
    println!("====================================");
    println!("This is a simple calculator that performs basic arithmetic operations.");
    println!("You can add, subtract, multiply, and divide two numbers.");
    println!("====================================");

    let first_value = prompt_for_number("Please enter the first number:");
    let second_value = prompt_for_number("Please enter the second number:");

    let operator = prompt_for_char("Please enter the operation you want to perform (+:add, -:sub, *:mult, /:div):");

    let result = match operator {
        '+' => calculate(first_value, second_value, Operator::Add),
        '-' => calculate(first_value, second_value, Operator::Sub),
        '*' => calculate(first_value, second_value, Operator::Mult),
        '/' => calculate(first_value, second_value, Operator::Div),
        _ => {
            println!("Invalid operator. Please use +, -, *, or /.");
            return;
        }
    };

    println!(
        "The result of {} {} {} is {:?}",
        first_value, operator, second_value, result
    );
}
