use crate::functions::mini_chain;

pub fn run_mini_chain() {
    println!("Chain running");
    mini_chain::simulator::chain_simulator();
}
