extern crate transactions;
extern crate vm;

use transactions::{Transaction};
use vm::instructions::{*, stack_types::*};


pub struct Block {
	blockHeader: BlockHeader,
	coinbase: Transaction,
	transactions: Vec<Transaction>
}


//TODO change stack types to include u16 and u32
struct BlockHeader {
	version: stack_types,			//16 bits
	prev_block_hash: stack_types,	//256 bits
	all_tx_hash: stack_types,		//256 bits
	nonce: stack_types,				//32 bits
}

impl Block {
	pub fn deserialize_block(block_bytes: Vec<stack_types>) {

	}

	pub fn serialize_block(block: Block) -> Result<Vec<u8>, String> {
		let mut sblock: Vec<u8> = Vec::new();
		if let bytes2(i) = block.blockHeader.version {
			for s in 0..i.len() {
				sblock.push(i[s]);
			}
		} else {
			return Err(String::from("Missing `version` from `BlockHeader`"));
		}

		return Ok(sblock);
	}
}