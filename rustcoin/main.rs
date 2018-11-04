extern crate transactions;
extern crate utils;
extern crate chain;
extern crate consensus;
extern crate rkv;

use rkv::{Manager, Rkv, Store, Value};
use std::fs;
use std::path::Path;

extern crate sha2;

use sha2::{Sha256, Digest};

use transactions::*;

fn main() {
	//init_chain();
	let mut s: Vec<u8> = vec![1,2,3,4];
	println!("{:?}", s[0..1].to_vec());
}

fn init_chain() {
	//construct_genesis();
}


fn construct_genesis() -> Vec<u8>{
	let mut version: Vec<u8> = vec![0,1];
	let mut prev_block_hash: Vec<u8> = vec![0;32];
	let mut all_tx_hash: Vec<u8> = vec![0;32];
	let mut nonce: Vec<u8> = vec![0;4];
	let mut coinbase_destination: Vec<u8> = vec![0;32];

	//construct the block
	let mut block: Vec<u8> = Vec::new();
	//block header
	block.append(&mut version);
	block.append(&mut prev_block_hash);
	block.append(&mut all_tx_hash);
	block.append(&mut nonce);

	//first tx (only coinbase)
	block.append(&mut coinbase_destination);

	consensus::insert_block(block.clone());
	return block;
}