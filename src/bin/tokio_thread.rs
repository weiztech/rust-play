extern crate core;

use core::task;
use tokio::time::{sleep, Duration};
use std::time::Instant;

async fn cpu_bound_task(id: usize, task_name: &str){
    let start = Instant::now();
    let mut num: usize = 0;
    println!("Task `{}` {}: Start", task_name, id);
    // Simulate CPU-bound work
    for _ in 0..5_000_000 {
        num += 2; // Dummy computation
    }
    println!(
        "Task `{}` {}: End after {:?} - Num {}",
        id,
        task_name,
        Instant::now().duration_since(start),
        num
    );
}

#[tokio::main] // Multi threaded runtime
async fn main() {
    let task_multi = "Multi Thread &str";
    let handles: Vec<_> = (1..=5)
        .map(|id| tokio::spawn(cpu_bound_task(id, task_multi))) // Spawns tasks across threads
        .collect();

    for handle in handles {
        handle.await.unwrap();
    };

    let task_multi_string = String::from("Multi Thread String");
    let handles2: Vec<_> = (1..=5)
        .map(|id| {
            let task_clone = task_multi_string.clone();
            tokio::spawn( async move {
                cpu_bound_task(id, task_clone.as_str()).await
            })
        }) // Spawns tasks across threads
        .collect();

    for handle in handles2 {
        handle.await.unwrap();
    };

    let texts = vec![
        String::from("Vec String - 1st"),
        String::from("Vec String - 2nd"),
        String::from("Vec String - 3rd"),
        String::from("Vec String - 4nd"),
        String::from("Vec String - 5nd"),
    ];
    let mut tasks: Vec<_> = vec![];
    for (id, text) in texts.iter().enumerate() {
        let task_clone = text.clone();
        // println!("Task Debug `{}` {}", id, task_clone);
        tasks.push(
            tokio::spawn( async move {
                cpu_bound_task(id + 1, task_clone.as_str()).await;
            })
        );
    };

    for task in tasks {
        task.await.unwrap();
    }
}
