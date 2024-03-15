use std::io;
use crate::functions::mythreading;

pub fn threading_interface() {
    let mut input = String::new();

    println!("Enter the number of factories:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input for number of factories.");
    let num_factories: i32 = input.trim().parse().expect("Please enter a valid number.");
    input.clear();

    println!("Enter the max value of range:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input for max value of range.");
    let max_value: i32 = input.trim().parse().expect("Please enter a valid number.");

    mythreading::thread_factory(num_factories, max_value);
}
