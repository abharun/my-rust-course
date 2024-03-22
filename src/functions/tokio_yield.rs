use tokio::try_join;

pub async fn first_task_with_yield(limit: i32) -> Result<(), String> {
    for i in 0..limit {
        println!("Task 1 timestamp is #{i}");
        tokio::task::yield_now().await;
    }

    Ok(())
}

pub async fn second_task_with_yield(limit: i32) -> Result<(), String> {
    for i in 0..limit {
        println!("Task 2 timestamp is #{i}");
        tokio::task::yield_now().await;
    }

    Ok(())
}

pub async fn run_tasks_simultaneously(first_limit: i32, second_limit: i32) {
    try_join!(first_task_with_yield(first_limit), second_task_with_yield(second_limit))
        .expect("Running tasks failed!");
}
