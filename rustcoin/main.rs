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
	let x = VM::new([PUSH4, 1, 44,44,44,PUSH1, 4,GET_UTXO].to_vec());
	println!("{:?}", x.execute());
}

