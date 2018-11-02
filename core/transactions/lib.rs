//TODO: Need to change push opcode so that different amounts can be pushed onto the stack.
//e.g. push32, push2, push1
 
extern crate vm;

use vm::instructions::{*, stack_types::*};

pub struct Transaction {
	inputs: Input,
	outputs: UTXO
}

pub struct UTXO {
	owner_pub_key_hash: u32,
	amount: u32,
	lockScript: Vec<u8> //DUP HASH160 EQUAL_VERIFY CHECKSIG
}

pub struct Input {
	pub tx_hash: stack_types, //Refers to utxo
	pub index: stack_types,
	pub from_pub_key: stack_types,
	pub signature: stack_types,
	pub unlockScript: stack_types //PUSH <signature> PUSH <from_pub_key>
}

//PUSH <to>
//PUSH <amount>
//PUSH <unlock_script>									
//PUSH <signature> PUSH <from_pub_key>  //load the sig and pub key of spender
//PUSH <utxo_index> 	
//PUSH <tx_hash>	
//GET_UTXO								//okay now we have the utxo saved in memory
//START
//DUP HASH160 							//get hash of inputed pubkey
//EQUAL_VERIFY 							//load the utxo from memory and check pubkey hash matches; throw error if false
//CHECKSIG								//Check the signature of the inputed pubkey matches the signature provided; throw error if false
//END									//end check if stack == 1
										//if yes pop 1 and create new utxo w/ unlock script, amount and to and delete utso stored in memory

impl Transaction {
	pub fn serialize(&self) {

	}

	pub fn serialize_input(input: Input) -> Result<Vec<u8>, String> {
		let mut sinput: Vec<u8> = Vec::new();

		sinput.push(0x03); //PUSH -- WONT WORK
		if let bytes32(i) = input.signature {
			for s in 0..i.len() {
				sinput.push(i[s]); //Push the signature on the stack
			}
		} else {
			return Err(String::from("Error serializing transaction: `singature` invalid"));
		}

		sinput.push(PUSH); //PUSH -- WONT WORK
		if let bytes32(i) = input.from_pub_key {
			for s in 0..i.len() {
				sinput.push(i[s]); //Push the from_pub_key onto the stack
			}
		} else {
			return Err(String::from("Error serializing transaction: `from_pub_key` invalid"));
		}

		sinput.push(PUSH); //PUSH -- WONT WORK SEE TOP
		if let bytes2(i) = input.index {
			for s in 0..i.len() {
				sinput.push(i[s]); //Push the output index inside the utxo to be spent on the stack
			}
		} else {
			return Err(String::from("Error serializing transaction: `index` invalid"));
		}

		sinput.push(0x03); //PUSH -- WONT WORK
		if let bytes32(i) = input.tx_hash {
			for s in 0..i.len() {
				sinput.push(i[s]); //Push the tx_hash onto the stack
			}
		} else {
			return Err(String::from("Error serializing transaction: `tx_hash` invalid"));
		}

		sinput.push(GET_UTXO);  

		return Ok(sinput);
	}

	fn serialize_output(&self) -> Vec<u8> {
		vec![]
	}
}