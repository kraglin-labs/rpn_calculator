use rpn_calculator::*;

fn main() {
    let expression = "5 1 2 + 4 * + 3 -";

    // Validate the expression
    if is_rpn(expression) {
        let result = mathik(expression);
        println!("Result: {:?}", result);
    } else {
        println!("Invalid RPN expression!");
    }
}
