//OP_CODES
pub const START: u8 = 0x01; 			//Start of unlock script
pub const END: u8 = 0x02;   			//End of UTXO lock script && therefore end of script - if 1 is on the stack mark UTXO as spent and create new utxos
pub const GET_UTXO: u8 = 0x03;		//Get the utxo and save it in MSTORE memory
pub const DUP_HASH160: u8 = 0x04;	//Hash the top item on the stack and push it onto the stack
pub const EQUAL_VERIFY: u8 = 0x05;	//Check that two hashes are equal and pop the last two elements. Terminate if false
pub const CHECKSIG: u8 = 0x06;		//Verify that the top pubkey on the stack matches the second top signature on the stack then pop both and push 1
pub const PUSH1: u8 = 0x07;	 		//PUSH 1 byte
pub const PUSH2: u8 = 0x08;			//PUSH 2 bytes
pub const PUSH4: u8 = 0x09;			//PUSH 4 bytes
pub const PUSH32: u8 = 0x0a;	 	//PUSH 32 bytes
