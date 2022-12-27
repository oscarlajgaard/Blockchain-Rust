
use sha256::digest;

pub struct TransactionData {
    pub s_key: String,
    pub r_key: String,
    pub amt: f64,
}

impl TransactionData {
    pub fn new(sender: String, recv:String, amt:f64) -> TransactionData {
        TransactionData {
            s_key: sender,
            r_key: recv,
            amt: amt,
        }
    }

    pub fn get_hash(&self) -> String {
        digest(format!("{}{}{}", self.s_key, self.r_key, self.amt.to_string()))
    }

    pub fn get_details(&self) {
        println!("\t\tSender: {}", self.s_key);
        println!("\t\tReciever: {}", self.r_key);
        println!("\t\tAmount: {}", self.amt);
    }

}

