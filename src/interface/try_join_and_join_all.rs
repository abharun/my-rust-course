use crate::functions::try_join_and_join_all;
use std::io;

pub async fn try_join_and_join_all_interface() {
    println!("Enter the number of threads:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number of threads");

    let counts = input.trim().parse::<i32>().unwrap();

    try_join_and_join_all::run_threads(counts).await;
}
