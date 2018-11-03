//Consensus provides handlers for the verification of;
// - new blocks
// - new transactions 
//and writes them to the blockchain db or the mempool
//TODO -- Verify signature in transaction (line 88)
//TODO -- need to store verified tx in memory
extern crate vm;
extern crate blocks;
extern crate utils;

use vm::instructions::{*};

///Verify new blocks that come in and write to db
pub fn verify_new_block(block: Vec<u8>) -> Result<bool, String> {
	let mut block_header: [u8; 70] = [0; 70];
	block_header.copy_from_slice(&block[0..70]); //Get blockheader into array
	let version = &block_header[0..2];

	//Version check
	if version != [0,1] {
		return Err(String::from("ERROR: VERIFY BLOCK: Incompatable `version`"));
	}

	//Nonce check
	if !utils::hash_satisfies_difficulty(&block) {
		return Err(String::from("ERROR: VERIFY BLOCK: Invalid `nonce`"));
	}

	//Matches with previous block check
	let prev_block_hash = &block_header[2..34];
	if prev_block_hash != utils::get_prev_block_hash() {
		return Err(String::from("ERROR: VERIFY BLOCK: `prev_block_hash` does not match"));
	}

	//Verify tx_hash matches hash of all tx
	let tx_hash = &block_header[34..66];
	let all_tx_bytes = block_header[70..].to_vec();
	if tx_hash != utils::hash_tx(all_tx_bytes.clone()) {
		return Err(String::from("ERROR: VERIFY BLOCK: `tx_hash` does not match"));
	}

	//Verify all tx
	let mut program_counter: usize = 0;
	let mut valid_tx_vector: Vec<Vec<u8>>  = Vec::new();
	while program_counter != block.len() {
		match verify_tx(all_tx_bytes[program_counter..].to_vec(), true) { 
			Ok(i) => { program_counter = i; }, 
			Err(e) => { return Err(e); }
		}
	}

	return Ok(true);
}

fn verify_coinbase(coinbase_tx: [u8; 70]) {

}

///Transaction format inside each block
//version no - 4 bytes
//input count - 1 byte
//-(input count times)
//--Transaction hash
//--Output index 4 bytes
//--unlock script size in bytes - 1 bytes
//--unlock script 
//-
//output count - 6 bytes
//-(output count times)
//--value - 6 bytes
//--pubkey (to address) //32 bytes

//todo verify the all the transactions
fn verify_tx(all_tx_bytes: Vec<u8>, is_Block: bool) -> Result<usize, String> {
	let version = &all_tx_bytes[0..4];
	if version != [0,0,0,1] {
		return Err(String::from("VERIFY TX ERROR: Incompatable `version` in tx"));
	}

	let input_count = all_tx_bytes[4];
	let mut sum_inputs: u64 = 0;
	let mut s: usize = 4; //counter for position in bytecode of block
	for i in 0..input_count {
		let utxo_tx_hash = &all_tx_bytes[s..s+32];
		s+=32;
		let utxo_index = all_tx_bytes[s];
		s+=1;

		//---------------- TODO
		//load utxo
		//if signature(hash(utxo)) == utxo pub key then valid
		//if fail throw error
		//add sum inputs
		s+=32; //increase by 32 because of signature (in reality this will probs be 67-70 bytes)
	}

	let output_count = all_tx_bytes[s];
	s+=1;
	let mut sum_outputs: u64 = 0; //Amount to be spent
	for i in 0..output_count {
		let value_array = &all_tx_bytes[s..s+6];
		s+=6;
		for i in 0..value_array.len() { //Get the byte array of `sum_outputs` and cast it to a u32
			sum_outputs = sum_outputs * 16 + value_array[i] as u64;
		}

		if sum_outputs > sum_inputs {
			return Err(String::from("Sum of outputs exceeds the inputs"));
		}

		//TODO -- Need to store each output in memory array
		let to_pub_key = &all_tx_bytes[s..s+32];
		s+=32; 
	}

	if is_Block {
		//write utxo to LMDB
	} else {
		//write utxo to mempool
	}

	return Ok(s); //Return the program counter inside the block
}



