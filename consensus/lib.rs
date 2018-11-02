//Consensus provides handlers for the verification of;
// - new blocks
// - new transactions 
//and writes them to the blockchain db or the mempool
extern crate vm;
extern crate blocks;
extern crate utils;

use vm::instructions::{*, stack_types::*};

pub fn verify_new_block(block: Vec<u8>) -> Result<bool, String> {
	let mut block_header: [u8; 70] = [0; 70];
	block_header.copy_from_slice(&block[0..70]); //Get blockheader into array
	let version = &block_header[0..2];
	if version != [0,1] {
		return Err(String::from("ERROR: VERIFY BLOCK: Incompatable `version`"));
	}

	if !hash_satisfies_difficulty(&block) {
		return Err(String::from("ERROR: VERIFY BLOCK: Invalid `nonce`"));
	}

	let prev_block_hash = &block_header[2..34];
	if prev_block_hash != utils::get_last_block_hash() {
		return Err(String::from("ERROR: VERIFY BLOCK: `prev_block_hash` does not match"));
	}

	return Ok(true);
}

fn hash_satisfies_difficulty(block: &Vec<u8>) -> bool {
	return true;
}

/*
	version: stack_types,			//2 byte
	prev_block_hash: stack_types,	//32 byte
	all_tx_hash: stack_types,		//32 byte
	nonce: stack_types,				//4 byte
*/