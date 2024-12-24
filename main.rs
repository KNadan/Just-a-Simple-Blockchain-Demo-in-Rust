mod block;
mod blockchain;
mod wallet;
mod client;
mod file_saver;
mod transaction_logger;

use blockchain::Blockchain;
use wallet::Wallet;
use client::validate_client;
use file_saver::save_client_data;
use transaction_logger::log_transaction;
use std::io;

fn main() {
    let mut blockchain = Blockchain::new("Validator Node".to_string());
    let mut wallet = Wallet::new();

    loop {
        println!("\n--- Welcome to the Mini Blockchain Demo --- \n--- © 2024 KNadan ---\n");
        println!("Choose an option:\n");
        println!("1. Register a client (ID: 1000-9999)");
        println!("2. Make a transaction");
        println!("3. View the blockchain");
        println!("4. Validate the blockchain");
        println!("5. Check client balance");
        println!("6. View instructions");
        println!("7. Exit\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut client_id = String::new();
                println!("Enter a client ID (1000-9999):");
                io::stdin().read_line(&mut client_id).unwrap();
                if validate_client(client_id.trim()) {
                    wallet.create_wallet(client_id.trim());
                    save_client_data(client_id.trim());
                    println!(
                        "Client {} registered successfully with a balance of 100 tokens!",
                        client_id.trim()
                    );
                } else {
                    println!("Invalid client ID. Please try again.");
                }
            }
            "2" => {
                let mut sender = String::new();
                let mut recipient = String::new();
                let mut amount = String::new();

                println!("Enter sender ID:");
                io::stdin().read_line(&mut sender).unwrap();

                println!("Enter recipient ID:");
                io::stdin().read_line(&mut recipient).unwrap();

                println!("Enter amount:");
                io::stdin().read_line(&mut amount).unwrap();

                if wallet.transfer(sender.trim(), recipient.trim(), amount.trim().parse::<u64>().unwrap()) {
                    let transaction = format!(
                        "{} sent {} tokens to {}",
                        sender.trim(),
                        amount.trim(),
                        recipient.trim()
                    );
                    blockchain.add_block(vec![transaction.clone()]);
                    log_transaction(sender.trim(), recipient.trim(), amount.trim().parse().unwrap());
                    println!("Transaction added successfully!");
                } else {
                    println!("Transaction failed. Check client IDs or balance.");
                }
            }
            "3" => {
                println!("\nBlockchain Data:");
                for block in &blockchain.chain {
                    println!(
                        "Block {}:\nIndex: {}\nTimestamp: {}\nTransactions: {:?}\nPrevious Hash: {}\nCurrent Hash: {}\nValidator: {}\n",
                        block.index, block.index, block.timestamp, block.transactions, block.previous_hash, block.current_hash, block.validator
                    );
                }
            }
            "4" => {
                if blockchain.is_valid() {
                    println!("Blockchain is valid!");
                } else {
                    println!("Blockchain is invalid!");
                }
            }
            "5" => {
                let mut client_id = String::new();
                println!("Enter client ID to check balance:");
                io::stdin().read_line(&mut client_id).unwrap();
                let balance = wallet.get_balance(client_id.trim());
                println!("Client {} has a balance of {} tokens.", client_id.trim(), balance);
            }
            "6" => {
                println!("\nInstructions:");
                println!("1. Register a client: Creates a new client with a default balance of 100 tokens.");
                println!("2. Make a transaction: Transfers tokens between registered clients.");
                println!("3. View the blockchain: Displays all blocks with detailed information.");
                println!("4. Validate the blockchain: Ensures the blockchain is valid and untampered.");
                println!("5. Check client balance: Displays the current balance of a client.");
                println!("6. View instructions: Shows this help message.");
                println!("7. Exit: Exits the program.");
            }
            "7" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
