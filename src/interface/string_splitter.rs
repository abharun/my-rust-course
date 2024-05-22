use std::io;
use crate::functions::string_splitter;

pub fn string_splitter_interface() {
    let mut input = String::new();

    println!("Enter the string to split:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input for first task limit.");
    let input_str = input.clone().trim().to_string();
    input.clear();

    println!("Enter the delimiter:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input for second task limit.");
    let delimiter = input.clone().trim().to_string();

    string_splitter::split_string(input_str.clone(), delimiter.clone());
}
