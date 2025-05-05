use crate::{
    calculate::{calculate, is_zero},
    operator::Operator,
};

#[test]
fn it_is_zero_integer() {
    let zero = 0;
    let result = is_zero(&zero);
    assert_eq!(result, true);
}

#[test]
fn it_is_zero_float_point() {
    let zero = 0.0;
    let result = is_zero(&zero);
    assert_eq!(result, true);
}

#[test]
fn it_is_not_zero_integer() {
    let zero = 1;
    let result = is_zero(&zero);
    assert_eq!(result, false);
}

#[test]
fn it_is_not_zero_float_point() {
    let zero = 0.1;
    let result = is_zero(&zero);
    assert_eq!(result, false);
}

#[test]
fn test_addition() {
    assert_eq!(calculate(2, 3, Operator::Add), Some(5));
    assert_eq!(calculate(2.5, 3.5, Operator::Add), Some(6.0));
}

#[test]
fn test_subtraction() {
    assert_eq!(calculate(5, 3, Operator::Sub), Some(2));
    assert_eq!(calculate(5.5, 3.5, Operator::Sub), Some(2.0));
}

#[test]
fn test_multiplication() {
    assert_eq!(calculate(4, 3, Operator::Mult), Some(12));
    assert_eq!(calculate(2.5, 2.0, Operator::Mult), Some(5.0));
}

#[test]
#[should_panic(expected = "Divsion by zero is not allowed")]
fn test_division_by_zero() {
    calculate(5, 0, Operator::Div);
}

#[test]
fn test_division() {
    assert_eq!(calculate(6, 3, Operator::Div), Some(2));
    assert_eq!(calculate(7.5, 2.5, Operator::Div), Some(3.0));
}
