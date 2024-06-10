use std::io::{self, BufRead, BufReader};
use crate::functions::sort_tree::run_cli;

pub fn sort_tree_interface() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    let mut lines = reader.lines();

    loop {
        let line = lines.next();
        match line {
            Some(Ok(txt)) => {
                if txt == "exit" {
                    break;
                }
                run_cli(txt);
            }
            Some(Err(err)) => {
                println!("Error Input: {:?}", err);
            }
            None => {
                break;
            }
        }
    }
}
