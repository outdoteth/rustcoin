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

extern crate openssl;

use openssl::bn::BigNumContext;
use openssl::ec::*;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::ecdsa::EcdsaSig;
use openssl::bn::BigNum;

fn main() {

	// get bytes from somewhere, i.e. this will not produce a valid key
	let public_key: Vec<u8> = vec![];

	// create an EcKey from the binary form of a EcPoint
	let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
	let mut ctx = BigNumContext::new().unwrap();
	let s = PointConversionForm::COMPRESSED;

	//109958066700685724622120501924555255086332095224636891443102527674161226090649
	//[3, 208, 43, 94, 15, 36, 97, 201, 192, 36, 32, 105, 77, 87, 206, 68, 227, 153, 31, 77, 108, 52, 178, 191, 242, 94, 15, 234, 15, 1, 137, 112, 235]
	let up = vec![3, 208, 43, 94, 15, 36, 97, 201, 192, 36, 32, 105, 77, 87, 206, 68, 227, 153, 31, 77, 108, 52, 178, 191, 242, 94, 15, 234, 15, 1, 137, 112, 235];
	let pstr: &str = &String::from("109958066700685724622120501924555255086332095224636891443102527674161226090649");
	let private_key = BigNum::from_dec_str(pstr).unwrap();
	let pubkey = EcPoint::from_bytes(&group, &up, &mut ctx);
	let skey2 = EcPoint::from_bytes(&group, &up, &mut ctx);
	let generator = EcPoint::new(&group);

	///Create new type EcKey on specified curve, with private key and pubkey
	let reco = &EcKey::from_private_components(&group, &private_key, &(pubkey.unwrap())).unwrap();
	let sig = EcdsaSig::sign(&[1], &reco).unwrap();
	println!("{:?}", &sig.r().to_vec().len());
	println!("{:?}", &sig.s().to_vec().len());
/*
	let x = reco.clone().public_key().to_bytes(&group, s, &mut ctx);
	let v = reco.clone().private_key().to_owned();
	println!("{:?}", x);
	println!("{:?}", v);

	let zv = EcKey::from_public_key(&group, &pubkey.unwrap()).unwrap();

	let t = sig.verify(&[1], &zv);
	println!("{:?}", t);*/


}


fn init_chain(x: &mut String) {
	x.push('a');
	println!("{:?}",x );
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