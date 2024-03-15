use my_rust_course::interface;
use std::io;

fn main() {
    println!("Hello, Rustaceans!\nWelcome to my Rust course to improve my Rust coding skills!\n");

    let mut stdin = io::stdin();
    let mut input = String::new();

    while let Ok(_) = stdin.read_line(&mut input) {
        let index: i32 = input.trim().parse().unwrap();
        if index == 1 {
            interface::threading_interface();
        }
    }
}
