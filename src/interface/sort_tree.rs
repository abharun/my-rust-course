use std::io::{self, BufRead, BufReader};
use crate::functions::sort_tree::{run_cli, SortTree};

pub fn sort_tree_interface() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    let mut lines = reader.lines();

    let mut tree = SortTree::default();

    loop {
        let line = lines.next();
        match line {
            Some(Ok(txt)) => {
                if txt == "exit" {
                    break;
                }
                run_cli(&mut tree, txt);
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
