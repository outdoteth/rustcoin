

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

///Transaction format inside each block
//version no - 4 bytes
//input count - 6 bytes
//-(input count times)
//--Transaction hash
//--Output index 4 bytes
//--unlock script size in bytes 2 bytes
//--unlock script 
//-
//output count - 6 bytes
//-(output count times)
//--version
//--value - 6 bytes
//--owner - 32 bytes

//This needs to be changed to match similar to bitcoin specs.
//The only thing that needs to go through the VM is the unlockScript and lockScript
//No need to overcomplicate things
impl Transaction {
	pub fn serialize(&self) {

	}

	/*pub fn serialize_tx(TX: Transaction) -> Result<Vec<u8>, String> {
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
	}*/

	fn serialize_output(&self) -> Vec<u8> {
		vec![]
	}
}