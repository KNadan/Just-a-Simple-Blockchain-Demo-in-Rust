use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_transaction(sender: &str, recipient: &str, amount: u64) {
    let timestamp = Utc::now().to_rfc3339(); // Add timestamp here
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("transactions.txt")
        .unwrap();

    writeln!(
        file,
        "[{}] Transaction: {} sent {} tokens to {}",
        timestamp, sender, amount, recipient
    )
    .unwrap();
}
