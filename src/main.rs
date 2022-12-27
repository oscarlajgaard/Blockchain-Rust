mod transaction;
mod block;
mod blockchain;





fn main() {

    let mut _b = blockchain::Blockchain::new();
    _b.add_block(
        block::Block::new(_b.get_prev(), // Prev hash
        chrono::offset::Local::now().to_string(), // timestamp
        transaction::TransactionData::new(String::from("Alex"),String::from("Kurt"), 10.0) // tx
        )
    );
    _b.add_block(
        block::Block::new(_b.get_prev(), // Prev hash
        chrono::offset::Local::now().to_string(), // timestamp
        transaction::TransactionData::new(String::from("Jens"),String::from("Thomas"), 135.0) // tx
        )
    );
    _b.add_block(
        block::Block::new(_b.get_prev(), // Prev hash
        chrono::offset::Local::now().to_string(), // timestamp
        transaction::TransactionData::new(String::from("Alex"),String::from("Karen"), 345.0) // tx
        )
    );
    _b.add_block(
        block::Block::new(_b.get_prev(), // Prev hash
        chrono::offset::Local::now().to_string(), // timestamp
        transaction::TransactionData::new(String::from("Karen"),String::from("Alex"), 1000.0) // tx
        )
    );

    _b.get_chain();

}


