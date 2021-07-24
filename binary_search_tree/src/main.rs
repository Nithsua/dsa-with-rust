use std::io::{self, Write};

struct Node<T> {
	key: T,
	left: Option<Box<Node<T>>>,
	right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
	fn new(key: T) -> Self {
		Node {
			key: key,
			left: None,
			right: None,
		}
	}
}

impl<T: PartialOrd + PartialEq> Node<T> {
	fn insert(&mut self, key: T) {
		if self.key > key {
			match self.left {
				Some(ref mut node) => node.insert(key),
				None => self.left = Some(Box::new(Node::new(key))),
			}
		} else {
			match self.right {
				Some(ref mut node) => node.insert(key),
				None => self.right = Some(Box::new(Node::new(key))),
			}
		}
	}

	fn search(&self, key: T) -> bool {
		if self.key == key {
			true
		} else if self.key < key {
			match self.right {
				Some(ref node) => node.search(key),
				None => false,
			}
		} else if self.key > key {
			match self.left {
				Some(ref node) => node.search(key),
				None => false,
			}
		} else {
			false
		}
	}

	// fn delete(&mut self, key: T) -> bool {
	// 	true
	// }
}

fn main() {
	let mut bst: Option<Box<Node<i32>>> = None;
	println!("Binary Search Tree");
	loop {
		let mut temp_string = String::new();
		print!("1.Insert\n2.Find\n0.Exit\nOption: ");
		io::stdout().flush().expect("Unable to flush the stdout");
		io::stdin()
			.read_line(&mut temp_string)
			.expect("Unable to read the input");
		let option = temp_string
			.trim()
			.parse::<u32>()
			.expect("Unabel to parse the choice");
		match option {
			1 => {
				print!("Enter the key to insert: ");
				io::stdout().flush().expect("Unable to flush the stdout");
				temp_string.clear();
				io::stdin()
					.read_line(&mut temp_string)
					.expect("Unable to read the key");
				let key = temp_string
					.trim()
					.parse::<i32>()
					.expect("Unable to parse the key");
				match bst {
					Some(ref mut node) => node.insert(key),
					None => bst = Some(Box::new(Node::new(key))),
				}
			}
			2 => {
				print!("Enter the key to search: ");
				io::stdout().flush().expect("Unable to flush the stdout");
				temp_string.clear();
				io::stdin()
					.read_line(&mut temp_string)
					.expect("Unable to read the key");
				let key = temp_string
					.trim()
					.parse::<i32>()
					.expect("Unable to parse the key");
				match bst {
					Some(ref node) => {
						if node.search(key) {
							println!("The key is present in the tree");
						} else {
							println!("The tree doesn't contain the key");
						}
					}
					None => println!("The tree is empty"),
				}
			}
			0 => (),
			_ => println!("Wrong option, take another look"),
		}
	}
}
