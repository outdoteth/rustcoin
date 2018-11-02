extern crate vm;
extern crate db;
extern crate transactions;
extern crate utils;

use vm::VM;
use vm::instructions::{*, stack_types::*};

use transactions::*;

fn main() {
	let x = vec![6; 15];
	let mut s: [u8; 10] = [0; 10];
	println!("{:?}", &x[0..2] == [6,6] );
}

