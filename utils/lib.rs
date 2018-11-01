//returns a block header for mining
pub fn get_block_template() -> [u8; 70] {
	let version = [0,1]; //2 bytes
	let prev_block_hash = get_last_block_hash();//need to fetch from db
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

pub fn get_last_block_hash() -> [u8; 32] {
	[0; 32]
}

//gets transactions from the mempool along with their collective hash
pub fn collect_tx_and_hash() -> ([u8; 32], Vec<u8>) {
	([0; 32], Vec::new())
}