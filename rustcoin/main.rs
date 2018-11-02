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
	let x = VM::new([PUSH1, 1, GET_UTXO, PUSH1, 7].to_vec());
	println!("{:?}", x.execute());
}

