extern crate vm;

use vm::VM;
use vm::instructions::{*, stack_types, stack_types::*};

fn main() {
	let pkey: [u8; 32] = [1; 32];
	let bytecode: Vec<stack_types> = vec![PUSH, byte(0x01), PUSH, bytes256(pkey)];
	let s = VM::new(bytecode);
	s.execute();    
}

