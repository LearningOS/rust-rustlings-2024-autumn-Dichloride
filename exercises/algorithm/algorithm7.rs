/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}

impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}

	fn is_empty(&self) -> bool {
		self.size == 0
	}

	fn len(&self) -> usize {
		self.size
	}

	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}

	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}

	fn pop(&mut self) -> Option<T> {
		if self.size > 0 {
			self.size -= 1;
			self.data.pop()
		} else {
			None
		}
	}

	fn peek(&self) -> Option<&T> {
		if self.size == 0 {
			return None;
		}
		self.data.get(self.size - 1)
	}
}

fn bracket_match(bracket: &str) -> bool {
	let mut stack = Stack::new();
	let matching_bracket = |c: char| -> Option<char> {
		match c {
			')' => Some('('),
			'}' => Some('{'),
			']' => Some('['),
			_ => None,
		}
	};

	for ch in bracket.chars() {
		match ch {
			'(' | '{' | '[' => stack.push(ch),
			')' | '}' | ']' => {
				if let Some(expected_open) = matching_bracket(ch) {
					if stack.pop() != Some(expected_open) {
						return false;
					}
				}
			}
			_ => (), // Ignore non-bracket characters
		}
	}

	// If stack is empty, all brackets were properly matched
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1() {
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s), true);
	}

	#[test]
	fn bracket_matching_2() {
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s), false);
	}

	#[test]
	fn bracket_matching_3() {
		let s = "{{([])}}";
		assert_eq!(bracket_match(s), true);
	}

	#[test]
	fn bracket_matching_4() {
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s), false);
	}

	#[test]
	fn bracket_matching_5() {
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s), false);
	}

	#[test]
	fn bracket_matching_6() {
		let s = "";
		assert_eq!(bracket_match(s), true);
	}
}
