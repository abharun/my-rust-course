use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::timeout;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub async fn echo_with_channel() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    let (tx, lx) = mpsc::channel::<String>();

    let _ = thread::spawn(move || {
        while let Ok(recv_str) = lx.recv() {
            if recv_str == String::from("bye") {
                break;
            }
            println!("Your Input: {:?}", recv_str);
        }
    });

    loop {
        let line = lines.next_line();
        match timeout(Duration::from_secs(10), line).await {
            Ok(Ok(Some(text))) => {
                let _ = tx.send(text.clone());
                if text == String::from("bye") {
                    break;
                }
            }
            Ok(Ok(None)) => {
                println!("No more input!");
                break;
            }
            Ok(Err(e)) => {
                println!("Error read line: {:?}", e.to_string());
                break;
            }
            Err(_) => {
                println!("No input within 10 seconds");
                break;
            }
        }
    }
}
