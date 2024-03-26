use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
struct MyThread {
    index: i32,
    value: i32,
}

impl MyThread {
    fn new(index: i32) -> Self {
        let mut rnd = rand::thread_rng();
        Self {
            index: index,
            value: rnd.gen_range(1..=50),
        }
    }

    async fn display_value(&self) -> Result<(), String> {
        sleep(Duration::from_millis(600)).await;
        loop {
            println!("Task #{:?} Value={:?}", self.index, self.value);
            sleep(Duration::from_secs(1)).await;
        }
    }
}

async fn extra_thread() {
    loop {
        println!("From extra thread!");
        sleep(Duration::from_secs(2)).await;
    }
}

pub async fn run_threads(thread_count: i32) {
    let mut threads = Vec::new();
    for i in 0..thread_count {
        threads.push(MyThread::new(i));
    }

    let mut thread_runners = Vec::new();
    for thrd in &threads {
        let thrd = thrd.clone();
        thread_runners.push(async move {
            thrd.display_value().await?;
            Ok::<(), String>(())
        });
    }

    let _ = tokio::try_join!(
        async {
            futures::future::try_join_all(thread_runners).await?;
            Ok::<(), String>(())
        },
        async {
            extra_thread().await;
            Ok::<(), String>(())
        }
    );
}
