pub mod instructions;
use instructions::*;

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
		let mut i: usize = 0;
		let mut STACK = self.STACK;
		let BINARY_STORE = self.BINARY_STORE;

		//loop through the BINARY_STORE and run the program
		while i < BINARY_STORE.len() {
			match BINARY_STORE[i] {
				START => {

				},
				PUSH1 => {
					i+=1;
					if BINARY_STORE.len() < i {
						return Err(format!("VM Error: PUSH1 stack overflow at position {}", i-1));
					}
					STACK.push(BINARY_STORE[i].clone());
				},
				PUSH2 => {
					i+=1;
					if BINARY_STORE.len() < i+1 {
						return Err(format!("VM Error: PUSH2 stack overflow at position {}", i-1));
					}
					for s in 0..2 {
						STACK.push(BINARY_STORE[i+s].clone());
					}
					i+=1;
				},
				PUSH4 => {
					i+=1;
					if BINARY_STORE.len() < i+3 {
						return Err(format!("VM Error: PUSH4 stack overflow at position {}", i-1));
					}
					for s in 0..4 {
						STACK.push(BINARY_STORE[i+s].clone());
					}
					i+=3;
				},
				PUSH32 => {
					i+=1;
					if BINARY_STORE.len() < i+31 {
						return Err(format!("VM Error: PUSH32 stack overflow at position {}", i-1));
					}
					for s in 0..32 {
						STACK.push(BINARY_STORE[i+s].clone());
					}
					i+=31;
				}
				//This is where verification of ownership of the utxo is handled
				GET_UTXO => {
					let tx_hash = STACK.pop();
					let tx_index = STACK.pop();
					//Now we need to search for the tx_hash, the tx_index in the found tx_hash,
					//And then append the lockScript to the BINARY_STORE
					//If it doesnt exist return error
				},
				DUP_HASH160 => {},
				EQUAL_VERIFY => {},
				CHECKSIG => {},
				END => {},
				_ => {}
			}
			i+=1;
		}
		println!("{:?}", STACK);
		self.BINARY_STORE = BINARY_STORE;
		self.STACK = STACK;
		return Ok(self);
	}
}