extern crate rkv;
extern crate tempfile;

use rkv::{Manager, Rkv, Store, Value};
use std::fs;
use tempfile::Builder;

pub fn initialize_test_db() {
	let root = Builder::new().prefix("simple-db").tempdir_in("./").unwrap();
	fs::create_dir_all(root.path()).unwrap();
	let path = root.path();
	println!("{:?}", path);
}