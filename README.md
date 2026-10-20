# rpn_calculator and tool

A simple Rust library for validating and evaluating mathematical expressions in Reverse Polish Notation (RPN) / Postfix Notation.

## Features

* **Tokenization**: Parses input strings into structured `Token` variants (`Number` or `Notation`).
* **Validation (`is_rpn`)**: Quickly checks if an expression is mathematically valid for RPN evaluation before processing.
* **Evaluation (`mathik`)**: Evaluates valid RPN expressions using stack operations. Supports standard binary operators (`+`, `-`, `*`, `/`).

## Usage

Add the functions to your code and pass any whitespace-separated RPN string:

```rust
use crate::{is_rpn, mathik};

fn main() {
    let expression = "5 1 2 + 4 * + 3 -";

    // Validate the expression
    if is_rpn(expression) {
        let result = mathik(expression);
        println!("Result: {:?}", result); // Output: [14.0]
    } else {
        println!("Invalid RPN expression!");
    }
}
