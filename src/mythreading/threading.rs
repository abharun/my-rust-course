use rand::{self, Rng};
use std::thread::{self, JoinHandle};

fn run_new_thread(index: i32, range: i32) -> JoinHandle<()> {
    thread::spawn(move || {
        for i in 1..=range {
            println!("Factory #{index}:{i}");
        }
    })
}

pub fn thread_factory(number: i32) {
    let mut rng = rand::thread_rng();

    let mut threads: Vec<JoinHandle<()>> = vec![];

    for index in 1..=number {
        let range = rng.gen_range(1..=100);
        threads.push(run_new_thread(index, range));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
