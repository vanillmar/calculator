mod calculate;
mod operator;

use calculate::calculate;
use operator::Operator;
use std::io;

fn main() {
    println!("Calculator App");
    println!("==================");
    println!("This is a simple calculator that performs basic arithmetic operations.");
    println!("You can add, subtract, multiply, and divide two numbers.");
    println!("==================");
    println!("==================");

    println!("Please enter first number:");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    let first_value = first_number
        .trim()
        .parse::<i32>()
        .expect("Please enter a valid number");

    println!("Please enter second number:");

    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    let second_value: i32 = second_number
        .trim()
        .parse::<i32>()
        .expect("Please enter a valid number");

    println!("Please enter the operation you want to perform (+:add, -:sub, *:mult, /:div)");
    let mut opt = String::new();
    io::stdin()
        .read_line(&mut opt)
        .expect("Failed to read the operator");

    let operator = opt
        .trim()
        .parse::<char>()
        .expect("Please enter a valid operator");

    match operator {
        '+' => println!(
            "The Sum of {} and {} is {:?}",
            first_number,
            second_number,
            calculate(first_value, second_value, Operator::Add)
        ),
        '-' => println!(
            "The Subtraction of {} and {} is {:?}",
            first_number,
            second_number,
            calculate(first_value, second_value, Operator::Sub)
        ),
        '*' => println!(
            "The Multiplication of {} and {} is {:?}",
            first_number,
            second_number,
            calculate(first_value, second_value, Operator::Mult)
        ),
        '/' => println!(
            "The Division of {} and {} is {:?}",
            first_number,
            second_number,
            calculate(first_value, second_value, Operator::Div)
        ),
        _ => {
            println!("Invalid operator. Please use +, -, *, or /.");
        }
    }
}
