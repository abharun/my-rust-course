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

fn main() {
    display_header();

    let mut stdin = io::stdin();
    let mut input = String::new();

    while let Ok(_) = stdin.read_line(&mut input) {
        let index = input.trim().parse();
        match index.unwrap() {
            0 => {
                break;
            }
            1 => {
                interface::threading_interface();
            }
            2 => {
                interface::echo_channel_interface();
            }
            _ => {
                println!("Not implemented for the index!");
            }
        };
        input.clear();
        display_func_seperator();
    }
}
