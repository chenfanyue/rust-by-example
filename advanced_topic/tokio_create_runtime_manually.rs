use tokio::time::{sleep, Duration};
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async_block());
}

async fn async_block() {
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
