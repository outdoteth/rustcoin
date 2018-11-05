//returns a block header for mining
extern crate chain;
extern crate sha2;
extern crate rkv;
extern crate openssl;

use openssl::bn::{BigNumContext, BigNum};
use openssl::ec::*;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::ecdsa::EcdsaSig;

use rkv::{Manager, Rkv, Store, Value};
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest}; //This algo should be changed to something asic resistant

//Template header for mining
pub fn get_block_template() -> Vec<u8> {
	let mut prev_block_hash = get_prev_block_hash();
	//let mut all_tx_hash = collect_tx_from_mempool(); //TODO: Collect tx from mempool
	//let mut tx_hash = hash(&all_tx_hash);
	let mut nonce = vec![0,0,0,0];

	let mut block_template: Vec<u8> = Vec::new(); //70 byte vector for block header
	block_template[0] = 0; // -- version
	block_template[1] = 1; // -- version
	block_template.append(&mut prev_block_hash);
	//block_template.append(&mut all_tx_hash); //TODO: Collect tx from mempool
	block_template.append(&mut nonce);
	return block_template; //this is just a block header
}

//Fetch last block hash 
pub fn get_prev_block_hash() -> Vec<u8> {
	let path = Path::new("./db/store");
	let created_arc = Manager::singleton().write().unwrap().get_or_create(path, Rkv::new).unwrap();
	let env = created_arc.read().unwrap();
	let store: Store = env.open_or_create_default().unwrap(); 
	let reader = env.read().expect("reader");
	if let Some(i) = reader.get(&store, vec![1]).unwrap() {
		match i {
			Value::Blob(s) => {return s.to_vec();},
			_ => {return vec![1];}
		}
	}
	return vec![1];
}

//Verifies an ecdsa signature for utxo spending
pub fn verify_signature(key: Vec<u8>, signature: Vec<u8>) {
	//need to convert the signature into two bignums (r and s)

	//Big num for context of storing keys
	let mut ctx = BigNumContext::new().unwrap();
	//
	let to_compressed = PointConversionForm::COMPRESSED;
	let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
	let pubkey = EcPoint::from_bytes(&group, &key, &mut ctx);
	//Set curve to secp256k1
	let wrapped_pub_key = EcKey::from_public_key(&group, &pubkey.unwrap()).unwrap();
	let sig_setup = EcdsaSig::from_private_components() //s and r
}

//gets transactions from the mempool 
//-- This needs access to the database
//This is used for block construction
pub fn collect_tx_from_mempool() -> Vec<u8> {
	Vec::new()
}

//Simple sha256 hash handler
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