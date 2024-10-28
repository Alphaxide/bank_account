use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn log_transaction(from_id:u32, to_id:u32, amount: f64) ->  io::Result<()> {
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("transaction_history.txt")?;

    writeln!(file, "Tranfer :from {} to {} - Amount : ${}", from_id, to_id, amount)
}