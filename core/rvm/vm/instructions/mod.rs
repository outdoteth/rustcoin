//Stack types
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum stack_types {
	byte(u8),			//OP_CODE
	bytes2([u8; 2]),
	bytes4([u8; 4]),		
	bytes32([u8; 32]),	//256 bit Values (e.g. result of a sha256 hash)
}

use self::stack_types::byte;

pub trait ByteUnwrap {
	fn unwrap(&self) -> u8;
}

impl ByteUnwrap for stack_types {
	fn unwrap(&self) -> u8 {
		match &self {
			byte(i) => *i,
			_ => {panic!("{:?}", "Invalid `OP_CODE` passed into `unwrap()`");},
		}
	}
}

//OP_CODES
pub const START: stack_types = byte(0x01); 			//Start of unlock script
pub const END: stack_types = byte(0x02);   			//End of UTXO lock script && therefore end of script - if 1 is on the stack mark UTXO as spent and create new utxos
pub const PUSH: stack_types = byte(0x03);	 		//PUSH item onto the stack
pub const GET_UTXO: stack_types = byte(0x04);		//Get the utxo and save it in MSTORE memory
pub const DUP_HASH160: stack_types = byte(0x05);	//Hash the top item on the stack and push it onto the stack
pub const EQUAL_VERIFY: stack_types = byte(0x06);	//Check that two hashes are equal and pop the last two elements. Terminate if false
pub const CHECKSIG: stack_types = byte(0x07);		//Verify that the top pubkey on the stack matches the second top signature on the stack then pop both and push 1