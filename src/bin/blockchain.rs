extern crate sha2;
extern crate rand;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;

use sha2::{Sha256, Digest};
use rand::{Rng, thread_rng};
use std::collections::BTreeMap;
use serde::Serialize;
use rmp_serde::Serializer;

const ALICE_ID: AccountId = 1;
const BOB_ID: AccountId = 2;
const LISA_ID: AccountId = 3;

// Default limit on random transactions
const MAX_AMOUNT: i64 = 3;
const BLOCK_SIZE_LIMIT: usize = 5;

fn main() {
    let balances = vec![
        (ALICE_ID, 50),
        (BOB_ID,   50),
    ].iter().cloned().collect();
    let mut state = State { balances };

    let genesis_contents = BlockContents {
        block_number: 0,
        parent_hash: None,
        transactions: vec![Transaction::from_btree(&state.balances)],
    };
    let genesis_block = Block::new(genesis_contents);

    let mut chain = vec![ genesis_block ];

    let mut txns = (0..30).map(|_| make_transaction(MAX_AMOUNT));

    'LOOP:
    loop {
        let mut txn_list = vec![];
        while txn_list.len() < BLOCK_SIZE_LIMIT {
            let txn = if let Some(t) = txns.next() { t } else { break 'LOOP };
            if txn.is_valid(&state) {
                state = state.update(&txn);
                txn_list.push(txn);
            } else {
                println!("ignored transaction");
                continue;
            }
        }

        let block = make_block(&txn_list, &chain);
        chain.push(block);
    }

    println!("Chain: {:?}", chain);
    for block in chain.iter() {
        println!("Block Number: {}", block.contents.block_number);

        print!("Hash: ");
        for val in block.hash.iter() {
            print!("{:x}", val);
        }
        println!("");

        if let Some(ref parent_hash) = block.contents.parent_hash {
            print!("Parent Hash: ");
            for val in parent_hash.iter() {
                print!("{:x}", val);
            }
            println!("");
        }

        for txn in block.contents.transactions.iter() {
            println!("  txn:");
            for (account, amount) in txn.amounts.iter() {
                println!("    {:04x}: {}", account, amount);
            }
        }

        println!("\n---\n");
    }

    println!("Final State:");
    for (account, bal) in state.balances.iter() {
        let name = match *account {
            ALICE_ID => "Alice".into(),
            BOB_ID => "Bob".into(),
            n => format!("0x{:04x}", n),
        };
        println!("  {}: {}", name, bal);
    }
}

fn make_block(transactions: &[Transaction], chain: &[Block]) -> Block {
    let parent = chain.last().unwrap();
    let block_number = parent.contents.block_number + 1;
    let contents = BlockContents {
        block_number,
        parent_hash: Some(parent.hash.clone()),
        transactions: transactions.iter().cloned().collect(),
    };
    Block::new(contents)
}

#[derive(Serialize, Clone, Debug)]
struct Block {
    pub hash: Vec<u8>,
    pub contents: BlockContents,
}

impl Block {
    pub fn new(contents: BlockContents) -> Self {
        let hash = contents.hash();
        Self { hash, contents }
    }
}

#[derive(Serialize, Clone, Debug)]
struct BlockContents {
    pub block_number: u64,
    pub parent_hash: Option<Vec<u8>>,
    pub transactions: Vec<Transaction>,
}

impl BlockContents {
    fn hash(&self) -> Vec<u8> {
        let mut buf = vec![];
        self.serialize(&mut rmp_serde::Serializer::new(&mut buf)).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(&buf);
        hasher.result().iter().cloned().collect()
    }
}

type AccountId = u64;

#[derive(Serialize, Clone, Debug)]
struct State {
    pub balances: BTreeMap<AccountId, i64>,
}

impl State {
    fn update(&self, txn: &Transaction) -> Self {
        let mut balances = self.balances.clone();
        for (key, val) in txn.amounts.iter() {
            let balance = balances.entry(*key).or_insert(0);
            *balance += *val;
        }

        State { balances }
    }
}

#[derive(Serialize, Clone, Debug)]
struct Transaction {
    pub amounts: BTreeMap<AccountId, i64>,
}

impl Transaction {
    pub fn from_vec(amounts: &Vec<(AccountId, i64)>) -> Self {
        let amounts: BTreeMap<AccountId, i64> = amounts.iter().cloned().collect();
        Self { amounts }
    }

    pub fn from_btree(amounts: &BTreeMap<AccountId, i64>) -> Self {
        let amounts = amounts.clone();
        Self { amounts }
    }

    pub fn is_valid(&self, state: &State) -> bool {
        if self.amounts.iter().fold(0, |accum, (_, bal)| accum + bal) != 0 {
            return false;
        }

        for (key, transferred) in self.amounts.iter() {
            let balance = state.balances.get(key).unwrap_or(&0);
            if balance + transferred < 0 {
                return false;
            }
        }
        true
    }
}

fn make_transaction(max_value: i64) -> Transaction {
    assert!(max_value > 0);
    let mut rng = thread_rng();

    let amount = rng.gen_range(1, max_value);
    let amount = if rng.gen() { -amount } else { amount };

    let mut amounts = BTreeMap::new();
    amounts.insert(ALICE_ID, amount);
    amounts.insert(BOB_ID, -amount);

    Transaction { amounts }
}
