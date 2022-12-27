

use crate::transaction::TransactionData;
use crate::block::Block;


pub fn create_genisis_block() -> Block {
    Block::new(
        String::from("0"), // Prev hash
        chrono::offset::Local::now().to_string(), // timestamp
        TransactionData::new(String::from("0"),String::from("0"), 0.0) // tx
    )
}



pub struct Blockchain {
    _chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain{_chain: vec![create_genisis_block()]}
    }
    pub fn get_chain(&self) {
        for val in self._chain.iter() {
            val.get_details();
        }
    }

    pub fn add_block(&mut self, mut _b: Block) {
        _b.generate_hash();
        self._chain.push(_b);
    }

    pub fn get_prev(&self) -> String {
        self._chain.last().expect("REASON").get_hash()
    }
}


