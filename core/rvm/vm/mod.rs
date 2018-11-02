pub mod instructions;
use instructions::*;

#[derive(Debug)]
pub struct VM {
	sc: u32, //Stack pointer -- Program pointer can be omitted since non-turing complete
	STACK: Vec<u8>,
	BINARY_STORE: Vec<u8>, //Store for the bytecode
	MSTORE: Vec<u8>
}

impl VM {
	pub fn new(bytecode: Vec<u8>) -> VM {
		Self {
			sc: 0,
			STACK: Vec::new(),
			BINARY_STORE: bytecode,
			MSTORE: Vec::new()
		}
	}

	//Runs the bytecode of the tx
	pub fn execute(mut self) -> Result<VM, String> {
		let mut STACK = self.STACK;

		//loop through the BINARY_STORE and run the block
		if let Err(i) = ::vm::VM::run_bytecode(self.BINARY_STORE.clone(), &mut STACK) {
			return Err(i);
		}

		println!("{:?}", STACK);
		self.STACK = STACK;
		return Ok(self);
	}

	fn run_bytecode(bytecode: Vec<u8>, STACK: &mut Vec<u8>) -> Result<bool, String> {
		let mut i: usize = 0;
		while i < bytecode.len() {
			match bytecode[i] {
				START => {

				},
				PUSH1 => {
					i+=1;
					if bytecode.len() < i {
						return Err(format!("VM Error: PUSH1 stack overflow at position {}", i-1));
					}
					STACK.push(bytecode[i].clone());
				},
				PUSH2 => {
					i+=1;
					if bytecode.len() < i+1 {
						return Err(format!("VM Error: PUSH2 stack overflow at position {}", i-1));
					}
					for s in 0..2 {
						STACK.push(bytecode[i+s].clone());
					}
					i+=1;
				},
				PUSH4 => {
					i+=1;
					if bytecode.len() < i+3 {
						return Err(format!("VM Error: PUSH4 stack overflow at position {}", i-1));
					}
					for s in 0..4 {
						STACK.push(bytecode[i+s].clone());
					}
					i+=3;
				},
				PUSH32 => {
					i+=1;
					if bytecode.len() < i+31 {
						return Err(format!("VM Error: PUSH32 stack overflow at position {}", i-1));
					}
					for s in 0..32 {
						STACK.push(bytecode[i+s].clone());
					}
					i+=31;
				}
				//This is where verification of ownership of the utxo is handled
				GET_UTXO => {
					let tx_hash = STACK.pop();
					let tx_index = STACK.pop();
					//Now we need to search for the tx_hash, the tx_index in the found tx_hash,
					//And then append the lockScript to the bytecode
					//If it doesnt exist return error
				},
				DUP_HASH160 => {},
				EQUAL_VERIFY => {},
				CHECKSIG => {},
				END => {},
				_ => {
					return Err(format!("VM ERROR: Invalid OP_CODE at position {}", i));
				}
			}
			i+=1;
		}
		return Ok(true);
	}
}