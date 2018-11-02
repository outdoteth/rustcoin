extern crate vm;
extern crate db;
extern crate transactions;
extern crate utils;
extern crate chain;

extern crate sha2;

use sha2::{Sha256, Digest};

use vm::VM;
use vm::instructions::{*};

use transactions::*;

fn main() {
	let v = VM::new([PUSH1, 1, PUSH4, 1].to_vec());
	let x = v.execute();
	if let Err(s) = x {
		println!("{:?}", s);
	}
}

