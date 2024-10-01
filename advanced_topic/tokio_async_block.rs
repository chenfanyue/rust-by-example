use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let task1 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 1 done");
    };

    let task2 = async {
        sleep(Duration::from_secs(2)).await;
        println!("Task 2 done");
    };

    tokio::join!(task1, task2);
    println!("All tasks done");
}
