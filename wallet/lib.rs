//create a wallet
extern crate utils;
extern crate consensus;

pub fn create_wallet() {

}

fn get_balance() {

}

//need to get block template
pub fn mine() {
	let block_template = utils::get_block_template();
	let mut block = block_template.clone();
	let mut coinbase_dest = vec![174, 132, 79, 187, 95, 178, 106, 70, 36, 139, 40, 103, 211, 176, 170, 144, 146, 226, 231, 133, 235, 189, 235, 173, 255, 86, 229, 100, 141, 70, 196, 241];
	block.append(&mut coinbase_dest);
	println!("{:?}",consensus::verify_new_block(block).unwrap() );
}