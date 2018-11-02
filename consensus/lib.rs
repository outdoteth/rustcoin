//Consensus provides handlers for the verification of;
// - new blocks
// - new transactions 
//and writes them to the blockchain db or the mempool
extern crate vm;
extern crate blocks;
extern crate utils;

use vm::instructions::{*, stack_types::*};

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
	let all_tx_bytecode = block_header[70..].to_vec();
	if tx_hash != utils::hash_tx(all_tx_bytecode.clone()) {
		return Err(String::from("ERROR: VERIFY BLOCK: `tx_hash` does not match"));
	}

	//Verify all tx
	let n_vm = vm::VM::new(all_tx_bytecode.clone());

	//Insert coinbase tx into utxo set



	return Ok(true);
}

fn verify_coinbase(coinbase_tx: [u8; 70]) {

}


//todo verify the all the transactions
fn verify_tx() -> Result<bool, String> {
	return Ok(true);
}

/*
	version: stack_types,			//2 byte
	prev_block_hash: stack_types,	//32 byte
	all_tx_hash: stack_types,		//32 byte
	nonce: stack_types,				//4 byte
*/