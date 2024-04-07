use crate::functions::utxo_tx_proc::{ Interface, Mempool};
use std::io::{self, BufRead};

pub fn utxo_tx_proc_interface() {
    println!("Input Tx Info:");
    let stdin = io::stdin();

    let mut pool = Mempool::default();

    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            if l == "exit" {
                break;
            }
            pool.run_cli(l);
        }
    }
}
