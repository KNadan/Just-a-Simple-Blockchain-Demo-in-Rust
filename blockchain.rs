use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub validator: String,
}

impl Blockchain {
    pub fn new(validator: String) -> Self {
        let genesis_block = Block::new(0, vec!["Genesis Block".to_string()], "0".to_string(), validator.clone());
        Blockchain {
            chain: vec![genesis_block],
            validator,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<String>) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            transactions,
            previous_block.current_hash.clone(),
            self.validator.clone(),
        );
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.current_hash {
                return false;
            }
        }
        true
    }
}
