mod calculate;
mod operator;

use calculate::calculate;
use operator::Operator;

fn main() {
    let x = 10;
    let y = 2;

    println!("{:?}", calculate(x, y, Operator::Add)); // Some(12)
    println!("{:?}", calculate(x, y, Operator::Sub)); // Some(8)
    println!("{:?}", calculate(x, y, Operator::Mult)); // Some(20)
    println!("{:?}", calculate(x, y, Operator::Div)); // Some(5)
}
