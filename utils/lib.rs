//returns a block header for mining
extern crate chain;
extern crate sha2;

use sha2::{Sha256, Digest}; //This algo should be changed to something asic resistant

//Template header for mining
pub fn get_block_template() -> [u8; 70] {
	let version = [0,1]; //2 bytes
	let prev_block_hash = get_prev_block_hash(); //need to fetch from db
	let (all_tx_hash, all_tx) = collect_tx_and_hash(); //need to fetch from mempool
	let nonce = [0,0,0,0];

	let mut block_template: [u8; 70] = [0; 70];
	block_template[0] = 0; // -- version
	block_template[1] = 1; // -- version
	for s in 0..prev_block_hash.len() {
		block_template[s+2] = prev_block_hash[s]; //prev hash
	}
	for s in 0..prev_block_hash.len() {
		block_template[s+34] = all_tx_hash[s]; //tx hash
	}
	return block_template;
}

//Last block hash 
//-- This needs access to the database
pub fn get_prev_block_hash() -> [u8; 32] {
	[0; 32]
}

pub fn hash_tx(txs: Vec<u8>) -> [u8; 32] {
	[0;32]
}

//gets transactions from the mempool along with their collective hash 
//-- This needs access to the database
pub fn collect_tx_and_hash() -> ([u8; 32], Vec<u8>) {
	([0; 32], Vec::new())
}

pub fn hash(digest: &Vec<u8>) -> Vec<u8> {
	let mut hasher = Sha256::new();
	hasher.input(&digest);
	let result = hasher.result();
	return result.to_vec();
}

//Check hash is less than difficulty
pub fn hash_satisfies_difficulty(block: &Vec<u8>) -> bool {
	let mut hasher = Sha256::new();
	hasher.input(block);
	let result = hasher.result();
	//TODO: `[255;32]` needs to be changed to chain::CHAIN_PARAMS::DIFFICULTY
	if result.as_slice() < &[255;32] {
		return true;
	}
	return false;
}