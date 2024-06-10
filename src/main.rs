use my_rust_course::interface;
use std::io;

fn display_header() {
    println!("==================================== 🦀 ====================================");
    println!("Hello, Rustaceans!\nWelcome to my Rust course to improve my Rust coding skills!");
    println!("You can see the index for particular functionality from README.md");
    println!("==================================== 🦀 ====================================");
    println!("Input func index:");
}

fn display_func_seperator() {
    println!("============================================================================");
    println!("Input func index:");
}

#[tokio::main]
async fn main() {
    display_header();

    let mut input = String::new();

    while let Ok(_) = io::stdin().read_line(&mut input) {
        if let Ok(index) = input.trim().parse::<i32>() {
            match index {
                0 => {
                    break;
                }
                1 => {
                    interface::threading_interface();
                }
                2 => {
                    interface::echo_channel_interface().await;
                }
                3 => {
                    interface::tokio_yield_interface().await;
                }
                4 => {
                    interface::try_join_and_join_all_interface().await;
                }
                5 => {
                    interface::hash_with_diff_interface();
                }
                6 => {
                    interface::utxo_tx_proc_interface();
                }
                7 => {
                    interface::string_splitter_interface();
                }
                8 => {
                    interface::sort_tree_interface();
                }
                _ => {
                    println!("Not implemented for the index!");
                }
            };
        } else {
            println!("Please input problem index as number!");
        }
        input.clear();
        display_func_seperator();
    }
}
