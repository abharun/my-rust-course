use std::io;
use crate::functions::hash_with_diff::calculate_hash;

pub fn hash_with_diff_interface() {
    let mut input = String::new();

    println!("Enter the text to hash:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input for number of factories.");
    let text = input.clone();
    input.clear();

    println!("Enter the hashing difficulty:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input for max value of range.");
    let difficulty: i32 = input.trim().parse().expect("Please enter a valid number.");

    let (hash_value, nonce) = calculate_hash(text.clone(), difficulty);
    println!("Text: {:?}\nDifficulty: {:?}\nNonce: {:?}\nHashValue: {:?}", text, difficulty, nonce, hash_value);
}