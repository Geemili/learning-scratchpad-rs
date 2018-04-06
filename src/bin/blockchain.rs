extern crate sha2;
#[macro_use] extern crate quicli;
extern crate rand;

use sha2::{Sha256, Digest};
use quicli::prelude::*;
use rand::{Rng, thread_rng};
use std::collections::HashMap;

#[derive(Debug, StructOpt)]
struct Cli {
}

const ALICE_ID: AccountId = 1;
const BOB_ID: AccountId = 2;
const LISA_ID: AccountId = 3;

main!(|_args: Cli| {
    let mut balances = HashMap::new();
    balances.insert(ALICE_ID, 5);
    balances.insert(BOB_ID, 5);
    let state = State { balances };

    println!("{}", Transaction::from_vec(&vec![(ALICE_ID, -3), (BOB_ID, 3)]).is_valid(&state));
    println!("{}", Transaction::from_vec(&vec![(ALICE_ID, -4), (BOB_ID, 3)]).is_valid(&state));
    println!("{}", Transaction::from_vec(&vec![(ALICE_ID, -6), (BOB_ID, 6)]).is_valid(&state));
    println!("{}", Transaction::from_vec(&vec![(ALICE_ID, -4), (BOB_ID, 2), (LISA_ID, 2)]).is_valid(&state));
    println!("{}", Transaction::from_vec(&vec![(ALICE_ID, -4), (BOB_ID, 3), (LISA_ID, 2)]).is_valid(&state));
});

type AccountId = u64;

struct State {
    pub balances: HashMap<AccountId, i64>,
}

impl State {
    fn update(&self, txn: Transaction) -> Self {
        let mut balances = self.balances.clone();
        for (key, val) in txn.amounts.iter() {
            let balance = balances.entry(*key).or_insert(0);
            *balance += *val;
        }

        State { balances }
    }
}

#[derive(Clone, Debug)]
struct Transaction {
    pub amounts: HashMap<AccountId, i64>,
}

impl Transaction {
    pub fn from_vec(amounts: &Vec<(AccountId, i64)>) -> Self {
        let amounts: HashMap<AccountId, i64> = amounts.iter().cloned().collect();
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

    let amount = rng.gen_range(0, max_value);
    let amount = if rng.gen() { -amount } else { amount };

    let mut amounts = HashMap::new();
    amounts.insert(ALICE_ID, amount);
    amounts.insert(BOB_ID, -amount);

    Transaction { amounts }
}
