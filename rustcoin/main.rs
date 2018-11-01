extern crate vm;
extern crate db;

use vm::VM;
use vm::instructions::{*, stack_types::*};

fn main() {
	let mut s: Vec<u8> = Vec::new();
	s.push(0x0);
	s.push(0x01);
	println!("{:?}", s);
}

