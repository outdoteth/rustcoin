//Consensus provides handlers for the verification of;
// - new blocks
// - new transactions
//and writes them to the blockchain db or the mempool
extern crate vm;
extern crate blocks;

use vm::instructions::{*, stack_types::*};

pub fn new_block(block: Vec<stack_types>) {

}