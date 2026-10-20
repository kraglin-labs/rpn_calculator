pub enum Token {
	Number(f64),
	Notation(String),
}

pub fn mathik(exp: &str) -> Vec<f64>{
	let all: Vec<String> = exp.split_whitespace().map(String::from).collect();
	let mut allfather: Vec<Token> = Vec::new();
	let mut stack: Vec<f64> = Vec::new();
	// Now, to consume all, and give life to the allfather
	for one in all {
		let gone = one.parse::<f64>();
		match gone {
			Ok(n) => allfather.push(Token::Number(n)),
			Err(_) => allfather.push(Token::Notation(one)),
		}
		
	}

	for one in &allfather {
		match one {
			Token::Number(num) => {
				stack.push(*num);
			},
			Token::Notation(stri) => {
				let first = stack.pop();
				let second = stack.pop();
				if let (Some(first), Some(second)) = (first, second) {
					match stri.as_str() {
						"+" => {
							stack.push(first + second);
						},
						"-" => {
							stack.push(second - first);
						},
						"*" => {
							stack.push(first * second);
						},
						"/" => {
							stack.push(second / first);
						},
						_ => (),						
					}
				}
			}
		}
	}

	stack
}

pub fn is_rpn(exp: &str) -> bool {
	// For this trick to work, it has to end with one
	let mut size = 0;

	for token in exp.split_whitespace() {
		if token.parse::<f64>().is_ok() {
			size += 1;
		} else if is_operator(token) {
			if size < 2 {
				return false;
			}
			size -= 1;
		} else {
			return false;
		}
	}

	size == 1
}

pub fn is_operator(stri: &str) -> bool {
	matches!(stri, "+" | "-" | "*" | "/")
}


#[cfg(test)]
mod tests {
	use crate::mathik;
	use crate::is_rpn;
	#[test]
	fn test_mathik() {
		let test = "5 1 2 + 4 * + 3 -";
		assert_eq!(mathik(test), [14.0]);
	}

	#[test]
	fn test_is_rpn() {
		let test = "5 1 2 + 4 * + 3 -";
		assert!(is_rpn(test));
	}
}
