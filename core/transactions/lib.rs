//TODO: Need to change push opcode so that different amounts can be pushed onto the stack.
//e.g. push32, push2, push1
 
extern crate vm;

use vm::instructions::{*};

pub struct Transaction {
	inputs: Input,
	outputs: UTXO
}

pub struct UTXO {
	owner_pub_key_hash: [u8; 32],
	amount: [u8; 4],
	lockScript: Vec<u8> //DUP HASH160 EQUAL_VERIFY CHECKSIG
}

pub struct Input {
	pub tx_hash: [u8; 32], //Refers to utxo
	pub index: [u8; 2],
	pub from_pub_key: [u8; 32],
	pub signature: [u8; 32],
	pub unlockScript: [u8; 32] //PUSH <signature> PUSH <from_pub_key>
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

	pub fn serialize_tx(TX: Transaction) -> Result<Vec<u8>, String> {
		let mut sinput: Vec<u8> = Vec::new();

		let input = TX.inputs;
		let output = TX.outputs;

		//PUSH32 <to>
		sinput.push(PUSH32);
		for i in 0..output.owner_pub_key_hash.len() {
			sinput.push(output.owner_pub_key_hash[i]);
		}

		//PUSH4 <amount>
		sinput.push(PUSH4);
		for i in 0..output.amount.len() {
			sinput.push(output.amount[i]);
		}

		/// --------------------- Unlock script --------------------- ///
		//PUSH32 <signature> 
		//---- TODO: This needs to be changed to ECDSA signature (67-70 bytes);
		sinput.push(PUSH32);
		for i in 0..input.signature.len() {
			sinput.push(input.signature[i]);
		}

		//PUSH32 <from>
		sinput.push(PUSH32);
		for i in 0..input.from_pub_key.len() {
			sinput.push(input.from_pub_key[i]);
		}
		/// --------------------- Unlock script --------------------- ///

		//PUSH <utxo index>
		sinput.push(PUSH2);
		for i in 0..input.index.len() {
			sinput.push(input.index[i]);
		}

		//PUSH <utxo hash>
		sinput.push(PUSH32);
		for i in 0..input.tx_hash.len() {
			sinput.push(input.tx_hash[i]);
		}

		//Get the utxo and run the given unlock script bytecode on top of the current VM (and stack)
		sinput.push(GET_UTXO);

		return Ok(sinput);
	}

	fn serialize_output(&self) -> Vec<u8> {
		vec![]
	}
}