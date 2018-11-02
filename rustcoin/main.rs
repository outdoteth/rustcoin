extern crate vm;
extern crate db;
extern crate transactions;
extern crate utils;
extern crate chain;

extern crate sha2;

use sha2::{Sha256, Digest};

use vm::VM;
use vm::instructions::{*, stack_types::*};

use transactions::*;

fn main() {
	let mut hasher = Sha256::new();
	hasher.input([6;8]);
	let result = hasher.result();
	println!("{:?}", result.as_slice() < &[255;32]);
}

