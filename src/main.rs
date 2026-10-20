// use std::io;
use rpn_calculator::*;

fn main() {
    let input = "5 1 2 + 4 * + 3 -";
    let answer = mathik(input);
    println!("input: {}", input);
    println!("answer: {:?}", answer);
}

