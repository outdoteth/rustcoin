extern crate vm;
extern crate db;
extern crate transactions;
extern crate utils;

use vm::VM;
use vm::instructions::{*, stack_types::*};

use transactions::*;

fn main() {
	let testx = Input {
			tx_hash: bytes32([0; 32]), //Refers to utxo
			index: bytes2([0; 2]),
			from_pub_key: bytes32([0; 32]),
			signature: bytes32([0; 32]),
			unlockScript: bytes32([0; 32])
		};
	let s = Transaction::serialize_input(testx);
	println!("{:?}", s);
}

