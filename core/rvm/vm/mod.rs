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

	//Runs the bytecode of the block
	pub fn execute(mut self) -> Result<VM, String> {
		let mut STACK = self.STACK;

		//loop through the BINARY_STORE and run the block
		let mut program_counter: usize = 0;
		let is_lock_script = false;
		while program_counter != self.BINARY_STORE.len() {
			let vm_res = ::vm::VM::run_bytecode(self.BINARY_STORE.clone(), &mut STACK, program_counter, is_lock_script);
			match vm_res {
				Ok(i) => {
					program_counter = i;
				},
				Err(i) => {
					return Err(i);
				}
			}
		}
		println!("{:?}", program_counter);

		//get return value and run through bytecode again
		//return value = pc counter
		//means we have to pass in pc counter again
		//program_counter+pc each time loop comes through
		self.STACK = STACK;
		return Ok(self);
	}

	//Takes in bytecode for a tx and runs it
	//If the given bytecode is not a lockScript, 
	//increment the counter and return the program counter within the block
	fn run_bytecode(bytecode: Vec<u8>, STACK: &mut Vec<u8>, pc: usize, lockScript: bool) -> Result<usize, String> {
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
					//let tx_hash = STACK.pop();
					//let tx_index = STACK.pop();
					//Now we need to search for the tx_hash, the tx_index in the found tx_hash,
					//And then append the lockScript to the bytecode
					//If it doesnt exist return error
					::vm::VM::run_bytecode([PUSH1, 9].to_vec(), STACK, i, true); //recursive call to run bytecode of lockScript
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

		let mut local_program_counter: usize = 0;
		if lockScript {
			local_program_counter = pc;
		} else {
			local_program_counter = pc+i;
		}
		return Ok(local_program_counter);
	}
}