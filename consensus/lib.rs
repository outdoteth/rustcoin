//Consensus provides handlers for the verification of;
// - new blocks
// - new transactions 
//and writes them to the blockchain db or the mempool
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
	verify_tx(all_tx_bytes);


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
//--size of scriptPubKey 2 bytes
//--scriptPubKey

//todo verify the all the transactions
fn verify_tx(all_tx_bytes: Vec<u8>) -> Result<bool, String> {
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
		//load utxo
		//if signature(hash(utxo)) == utxo pub key then valid
		//if fail throw error
		//add sum inputs
	}

	let output_count = all_tx_bytes[s];
	s+=1;
	for i in 0..output_count {
		let mut value: u32 = 0;
		let val_arr = &all_tx_bytes[s..s+6];
		s+=6;
		for i in 0..val_arr.len() {
			value = value * 16 + val_arr[i] as u32;
		}
	}

	return Ok(true);
}



