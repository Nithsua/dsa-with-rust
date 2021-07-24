use std::fmt;
use std::io::{self, Write};

struct Node<T> {
	key: T,
	next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
	fn new(key: T) -> Self {
		Node {
			key: key,
			next: None,
		}
	}

	fn from(key: T, next: Option<Box<Node<T>>>) -> Self {
		Node {
			key: key,
			next: next,
		}
	}
}

impl<T: PartialEq> Node<T>
where
	T: fmt::Display,
{
	fn search(&self, key: T) -> (bool, usize) {
		self.search_recursive(key, 0)
	}

	fn search_recursive(&self, key: T, mut count: usize) -> (bool, usize) {
		if self.key == key {
			return (true, count);
		} else {
			if let Some(node) = &self.next {
				count += 1;
				return node.search_recursive(key, count);
			}
			(false, 0)
		}
	}

	fn push(self, key: T) -> Option<Box<Node<T>>> {
		Some(Box::new(Node::from(key, Some(Box::new(self)))))
	}

	fn pop(self) -> Option<Box<Node<T>>> {
		self.next
	}

	fn display(&self) {
		print!("{}-> ", self.key);
		let mut ptr = &self.next;
		while let Some(node) = ptr {
			print!("{}-> ", node.key);
			ptr = &node.next;
		}
		println!("None");
	}
}

fn main() {
	let mut head: Option<Box<Node<i32>>> = None;
	println!("Nithsua Linked List Implementation");
	loop {
		let mut temp_string = String::new();
		print!("1.Push\n2.Pop\n3.Display\n4.Search\n0.Exit\nOption: ");
		io::stdout().flush().unwrap();
		io::stdin()
			.read_line(&mut temp_string)
			.expect("Unable to read the choice");
		let choice = temp_string
			.trim()
			.parse::<u32>()
			.expect("Unable to parse the choice");
		match choice {
			1 => {
				print!("Enter the value to push: ");
				io::stdout().flush().unwrap();
				temp_string.clear();
				io::stdin()
					.read_line(&mut temp_string)
					.expect("Unable to read the key");
				let key = temp_string
					.trim()
					.parse::<i32>()
					.expect("Unable to parse the key");
				if let Some(node) = head {
					head = node.push(key)
				} else {
					head = Some(Box::new(Node::new(key)));
				}
			}
			2 => {
				if let Some(node) = head {
					head = node.pop();
				} else {
					println!("The list is already empty");
				}
			}
			3 => {
				if let Some(ref node) = head {
					node.display();
				} else {
					println!("The list is empty");
				}
			}
			4 => {
				print!("Enter the value to search for: ");
				io::stdout().flush().unwrap();
				temp_string.clear();
				io::stdin()
					.read_line(&mut temp_string)
					.expect("Unable to read the input");
				let key = temp_string
					.trim()
					.parse::<i32>()
					.expect("Unable to parse the input");
				if let Some(ref node) = head {
					let (found, index) = node.search(key);
					if found {
						println!("The key {} is found at index {} of the list", key, index);
					} else {
						println!("The key {} is not list", key);
					}
				} else {
					println!("The list is empty");
				}
			}
			0 => {
				return;
			}
			_ => {
				println!("Wrong Option, check the options again...");
			}
		}
	}
}
