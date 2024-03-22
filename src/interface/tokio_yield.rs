use std::io;
use crate::functions::tokio_yield;

pub async fn tokio_yield_interface() {
    let mut input = String::new();

    println!("Enter the limit for first task:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input for first task limit.");
    let first_limit: i32 = input.trim().parse().expect("Please enter a valid number.");
    input.clear();

    println!("Enter the limit for second task:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input for second task limit.");
    let second_limit: i32 = input.trim().parse().expect("Please enter a valid number.");

    tokio_yield::run_tasks_simultaneously(first_limit, second_limit).await;
}
