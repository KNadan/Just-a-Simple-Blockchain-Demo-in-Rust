use std::collections::HashMap;

#[derive(Debug)]
pub struct Wallet {
    balances: HashMap<String, u64>,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            balances: HashMap::new(),
        }
    }

    pub fn create_wallet(&mut self, client_id: &str) {
        self.balances.entry(client_id.to_string()).or_insert(100); // Default balance
    }

    pub fn transfer(&mut self, sender: &str, recipient: &str, amount: u64) -> bool {
        let sender_balance = self.balances.entry(sender.to_string()).or_insert(0);
        if *sender_balance < amount {
            return false; // Insufficient funds
        }

        *sender_balance -= amount;
        let recipient_balance = self.balances.entry(recipient.to_string()).or_insert(0);
        *recipient_balance += amount;
        true
    }

    pub fn get_balance(&self, client_id: &str) -> u64 {
        *self.balances.get(client_id).unwrap_or(&0)
    }
}
