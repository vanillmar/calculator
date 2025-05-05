use core::panic;
use std::ops::{Add, Div, Mul, Sub};

use num_traits::Zero;

use crate::operator::Operator;

pub fn calculate<T>(a: T, b: T, operator: Operator) -> Option<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + Zero,
{
    match operator {
        Operator::Add => Some(a + b),
        Operator::Sub => Some(a - b),
        Operator::Mult => Some(a * b),
        Operator::Div => {
            if is_zero(&b) {
                panic!("Divsion by zero is not allowed");
            }
            Some(a / b)
        }
    }
}

fn is_zero<T: Zero>(value: &T) -> bool {
    value.is_zero()
}

#[cfg(test)]
mod tests;
