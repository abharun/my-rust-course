use my_rust_course::interface;
use std::io;

fn main() {
    println!("Hello, Rustaceans!\nWelcome to my Rust course to improve my Rust coding skills!\n");

    let mut stdin = io::stdin();
    let mut input = String::new();

    while let Ok(_) = stdin.read_line(&mut input) {
        let index = input.trim().parse();
        println!("{:?}", index);
        match index.unwrap() {
            1 => { interface::threading_interface(); },
            2 => { interface::echo_channel_interface(); }
            _ => {
                println!("Not implemented for the index!");
            }
        };
        input.clear();
    }
}
