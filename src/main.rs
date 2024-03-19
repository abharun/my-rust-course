use my_rust_course::interface;
use std::io;

fn main() {
    println!("Hello, Rustaceans!\nWelcome to my Rust course to improve my Rust coding skills!\nYou can see the index for particular functionality from README.md\nInput func index:");

    let mut stdin = io::stdin();
    let mut input = String::new();

    while let Ok(_) = stdin.read_line(&mut input) {
        let index = input.trim().parse();
        match index.unwrap() {
            1 => { interface::threading_interface(); },
            2 => { interface::echo_channel_interface(); }
            _ => {
                println!("Not implemented for the index!");
            }
        };
        input.clear();
        println!("Input func index:");
    }
}
