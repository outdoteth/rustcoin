extern crate transactions;

use transactions::{Transaction};


pub struct Block {
	blockHeader: BlockHeader,
	coinbase: Transaction,
	transactions: Vec<Transaction>
}


struct BlockHeader {
	version: [u8; 2],			//2 byte
	prev_block_hash: [u8; 32],	//32 byte
	all_tx_hash: [u8; 32],		//32 byte
	nonce: [u8; 4],				//4 byte
}


impl Block {

	//Deserialize block for viewing in cli? or gui?
	pub fn deserialize_block(&self, block_bytes: Vec<u8>) {

	}

	//Serialize block to be broadcast
	pub fn serialize_block(&self) -> Result<Vec<u8>, String> {
		let mut sblock: Vec<u8> = Vec::new();

		//First serialize the block-header
		//Version
		//Prev block hash
		//All tx hash
		//Nonce
		let blockHeader = &self.blockHeader;
		for i in 0..blockHeader.version.len() {
			sblock.push(blockHeader.version[i]);
		}
		for i in 0..blockHeader.prev_block_hash.len() {
			sblock.push(blockHeader.prev_block_hash[i]);
		}
		for i in 0..blockHeader.all_tx_hash.len() {
			sblock.push(blockHeader.all_tx_hash[i]);
		}
		for i in 0..blockHeader.nonce.len() {
			sblock.push(blockHeader.nonce[i]);
		}
		return Ok(sblock);
	}
}