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
use openssl::error::ErrorStack;

use rkv::{Manager, Rkv, Store, Value};
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest}; //This algo should be changed to something asic resistant

//Template header for mining
pub fn get_block_template() -> Result<Vec<u8>, String> {
	let mut prev_block_hash = get_prev_block_hash();
	let mut tx_in_mempool = collect_tx_from_mempool()?; //TODO: Collect tx from mempool
	let mut coinbase_dest = vec![174, 132, 79, 187, 95, 178, 106, 70, 36, 139, 40, 103, 211, 176, 170, 144, 146, 226, 231, 133, 235, 189, 235, 173, 255, 86, 229, 100, 141, 70, 196, 241];
	let mut digest: Vec<u8> = Vec::new();
	digest.append(&mut coinbase_dest);
	digest.append(&mut tx_in_mempool);
	let mut tx_hash = hash(&coinbase_dest); //just for testing this needs to be changed
	let mut nonce = vec![0,0,0,0];

	let mut block_template: Vec<u8> = Vec::new(); //70 byte vector for block header
	block_template.push(0);// -- version
	block_template.push(1); // -- version
	block_template.append(&mut prev_block_hash);
	block_template.append(&mut tx_hash); //TODO: Collect tx from mempool
	block_template.append(&mut nonce);
	return Ok(block_template); //this is just a block header
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
pub fn verify_signature(key: Vec<u8>, signature: Vec<u8>, message: Vec<u8>) -> Result<bool, ErrorStack>{
	//need to convert the signature into two bignums (r and s)
	//get the size of r and then r itself
	//then convert to bignum
	let r_size = signature[0] as usize;
	let r = &signature[1..r_size];
	let r_bignum = BigNum::from_slice(r)?;

	//get size of s and then s itself
	//then convert to bignum
	let s_size = signature[r_size] as usize;
	let s = &signature[r_size+1..s_size];
	let s_bignum = BigNum::from_slice(s)?;

	let mut ctx = BigNumContext::new()?; //Big num for context of storing keys
	let to_compressed = PointConversionForm::COMPRESSED; //Type of pub key
	let group = EcGroup::from_curve_name(Nid::SECP256K1)?; //All operations are done on secp256k1 curve
	let pubkey = EcPoint::from_bytes(&group, &key, &mut ctx)?; //Get a key and convert to EcPoint
	let wrapped_pub_key = EcKey::from_public_key(&group, &pubkey)?; //Wrap pubkey to EcKey type
	let sig_setup = EcdsaSig::from_private_components(r_bignum, s_bignum)?; //Merge r and s into Ecdsa sig type

	//Final check to make sure signature is valid
	let is_valid_signature = sig_setup.verify(&message, &wrapped_pub_key)?;
	return Ok(is_valid_signature);
}

//gets transactions from the mempool 
//-- This needs access to the database
//This is used for block construction
pub fn collect_tx_from_mempool() ->Result<Vec<u8>, String> {
	let path = Path::new("./db/store");
	let created_arc = Manager::singleton().write().unwrap().get_or_create(path, Rkv::new).unwrap();
	let env = created_arc.read().unwrap();
	let store: Store = env.open_or_create_default().unwrap(); 
	let reader = env.read().expect("reader");
	let tx_vec = match reader.get(&store, vec![1,2]).unwrap().unwrap() {
		Value::Blob(i) => i.to_vec(),
		_ => { return Err(String::from("ERROR: UTILS: No tx in mempool")); }
	};
	return Ok(tx_vec);
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
	return result.as_slice() < &[255;32];
}