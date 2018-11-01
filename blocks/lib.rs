extern crate transactions;
extern crate vm;

use transactions::{Transaction};
use vm::instructions::{*, stack_types::*};


pub struct Block {
	blockHeader: BlockHeader,
	coinbase: Transaction,
	transactions: Vec<Transaction>
}


struct BlockHeader {
	version: stack_types,			//2 byte
	prev_block_hash: stack_types,	//32 byte
	all_tx_hash: stack_types,		//32 byte
	nonce: stack_types,				//4 byte
}


impl Block {
	pub fn deserialize_block(&self, block_bytes: Vec<stack_types>) {

	}

	pub fn serialize_block(&self) -> Result<Vec<u8>, String> {
		let mut sblock: Vec<u8> = Vec::new();
		let blockHeader = &self.blockHeader;
		if let bytes2(i) = blockHeader.version {
			for s in 0..i.len() {
				sblock.push(i[s]);
			}
		} else {
			return Err(String::from("Missing `version` from `BlockHeader`"));
		}
		if let bytes32(i) = blockHeader.prev_block_hash {
			for s in 0..i.len() {
				sblock.push(i[s]);
			}
		} else {
			return Err(String::from("Missing `prev_block_hash` from `BlockHeader`"));
		}
		if let bytes32(i) = blockHeader.all_tx_hash {
			for s in 0..i.len() {
				sblock.push(i[s]);
			}
		} else {
			return Err(String::from("Missing `all_tx_hash` from `BlockHeader`"));
		}
		if let bytes4(i) = blockHeader.nonce {
			for s in 0..i.len() {
				sblock.push(i[s]);
			}
		} else {
			return Err(String::from("Missing `nonce` from `BlockHeader`"));
		}
		return Ok(sblock);
	}
}