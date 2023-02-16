use sha2::{Digest, Sha256};
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    fn new(
        index: u32,
        timestamp: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        nonce: u32,
    ) -> Block {
        let block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce,
        };
        let hash = block.hash_block();
        Block {
            hash,
            ..block
        }
    }

    fn hash_block(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.nonce
        );
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block #{} [Hash: {}, Prev. Hash: {}, Nonce: {}]",
            self.index, self.hash, self.previous_hash, self.nonce
        )
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            chain: vec![Block::new(
                0,
                0,
                vec![],
                String::from("0"),
                0,
            )],
        }
    }

    fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    fn is_chain_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if i > 0 && block.previous_hash != self.chain[i - 1].hash {
                return false;
            }
            if block.hash_block() != block.hash {
                return false;
            }
        }
        true
    }

    fn hash_last_block(&self) -> String {
        self.chain.last().unwrap().hash_block()
    }

    fn mine_block(&mut self, transactions: Vec<Transaction>) -> Block {
        let index = self.chain.len() as u32;
        let previous_hash = self.hash_last_block();
        let mut nonce = 0;
        let timestamp = chrono::Utc::now().timestamp();
        let mut block = Block::new(index, timestamp as u64, transactions, previous_hash, nonce);
        while &block.hash[0..2] != "00" {
            nonce += 1;
            block.nonce = nonce;
            block.hash = block.hash_block();
        }
        block
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    let transactions = vec![
        Transaction {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 1.0,
        },
        Transaction {
            sender: String::from("Bob"),
            receiver: String::from("Charlie"),
            amount: 2.0,
        },
    ];
    let block = blockchain.mine_block(transactions);
    blockchain.add_block(block);

    let mut other_blockchain = Blockchain::new();

    other_blockchain.add_block(Block::new(
        0,
        0,
        vec![],
        String::from("0"),
        0,
    ));
    other_blockchain.add_block(Block::new(
        1,
        0,
        vec![Transaction {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 1.0,
        }],
        other_blockchain.hash_last_block(),
        0,
    ));

    println!("Blockchain:");
    for block in &blockchain.chain {
        println!("{}", block);
    }
    println!("Is chain valid? {}", blockchain.is_chain_valid());

    println!("Other blockchain:");
    for block in &other_blockchain.chain {
        println!("{}", block);
    }
    println!("Is chain valid? {}", other_blockchain.is_chain_valid());
}


//This code creates a blockchain, adds a block with two transactions to it, 
//creates another blockchain, adds two blocks to it (the first one being the same as the genesis block), 
//and then prints both chains and whether they are valid or not.