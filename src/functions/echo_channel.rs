use std::io;
use std::sync::mpsc;
use std::thread;

pub fn echo_with_channel() {
    let mut input = String::new();

    let stdin = io::stdin();

    let (tx, lx) = mpsc::channel::<String>();

    let _ = thread::spawn(move || {
        while let Ok(recv_str) = lx.recv() {
            if recv_str == String::from("bye") {
                break;
            }
            println!("Your Input: {:?}", recv_str);
        }
    });

    while let Ok(_) = stdin.read_line(&mut input) {
        input = input.trim().to_string();
        tx.send(input.clone()).unwrap();
        if input == String::from("bye") {
            break;
        }
        input.clear();
    }
    println!("Echo function ended");
}
