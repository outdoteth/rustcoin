pub struct Transaction {
	inputs: Input,
	outputs: UTXO
}

struct UTXO {
	owner_pub_key_hash: u32,
	amount: u32,
	lockScript: Vec<u8> //DUP HASH160 EQUAL_VERIFY CHECKSIG
}

struct Input {
	tx_hash: u32, //Refers to utxo
	from_pub_key: u32,
	signature: u32,
	unlockScript: Vec<u8> //PUSH <signature> PUSH <from_pub_key>
}

//PUSH <to>
//PUSH <amount>
//PUSH <unlock_script>									
//PUSH <signature> PUSH <from_pub_key>  //load the sig and pub key of spender
//PUSH <tx_hash>
//PUSH <utxo_index> 		
//GET_UTXO								//okay now we have the utxo saved in memory
//START
//DUP HASH160 							//get hash of inputed pubkey
//EQUAL_VERIFY 							//load the utxo from memory and check pubkey hash matches; throw error if false
//CHECKSIG								//Check the signature of the inputed pubkey matches the signature provided; throw error if false
//END									//end check if stack == 1
										//if yes pop 1 and create new utxo w/ unlock script, amount and to and delete utso stored in memory

impl Transaction {
	fn serialize(&self) {

	}

	fn serialize_input(&self) -> Vec<u8> {
		vec![]
	}

	fn serialize_output(&self) -> Vec<u8> {
		vec![]
	}
}