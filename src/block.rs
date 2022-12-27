use crate::transaction::TransactionData;
use sha256::digest;



pub struct Block {
    pub prev_hash: String,
    pub t_time: String,
    pub tx: TransactionData,
    pub shash: String,
    nonce: u128,
}

pub fn spacer(i: i64) -> String {
    let mut ss: String = String::from("");
    for _k in 0..i {
        ss.push('#');
    }

    return ss;
}


impl Block {
    pub fn new(phash: String, time: String, trans: TransactionData) -> Block {
        Block{
            prev_hash: phash,
            t_time: time,
            tx: trans,
            shash: "0".to_string(),
            nonce: 2095684001, // burde være 0
        }
    }

    pub fn generate_hash(&mut self) {
        let mut _hash = digest(format!("{}{}{}{}", self.prev_hash, self.t_time, self.tx.get_hash(), self.nonce));
        println!("Mining block...");
        while hex::encode(&_hash) >= "30303030303030303030313964363638396330383561653136353833316539333466663736336165343661326136633137326233663162363061386365323666".to_string() {
            self.nonce += 1;
            _hash = digest(format!("{}{}{}{}", self.prev_hash, self.t_time, self.tx.get_hash(), self.nonce));
            print!("Trying hash: {} with nonce: {}\r", hex::encode(&_hash), self.nonce);
        }
        println!("Block sucessfully mined!");
        self.shash = _hash;
        
    }

    pub fn get_hash(&self) -> String {
        let rtn = &self.shash;
        return rtn.to_string();
    }

    

    pub fn get_details(&self) {
        println!("!======[ {} ]======!", self.get_hash());
        println!("\tprev_hash : {}", self.prev_hash);
        println!("\ttimestamp : {}", self.t_time);
        println!("\tTX_DATA:");
        self.tx.get_details();
        println!("!======[ {} ]======!", spacer(64));
    }

}

