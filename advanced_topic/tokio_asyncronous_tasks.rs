// main.rs 文件
use tokio::time::{sleep, Duration};

// 一个异步任务，模拟一些工作并返回一个数字
async fn task_one() -> i32 {
    println!("任务一开始执行...");
    // 模拟异步等待 2 秒
    sleep(Duration::from_secs(2)).await;
    println!("任务一完成！");
    10
}

// 另一个异步任务，模拟一些工作并返回一个字符串
async fn task_two() -> &'static str {
    println!("任务二开始执行...");
    // 模拟异步等待 3 秒
    sleep(Duration::from_secs(3)).await;
    println!("任务二完成！");
    "Hello, async world!"
}

#[tokio::main]
async fn main() {
    println!("开始执行异步任务...");

    // 并行执行两个异步任务
    let (result_one, result_two) = tokio::join!(task_one(), task_two());

    // 输出任务的返回结果
    println!("任务一的结果是: {}", result_one);
    println!("任务二的结果是: {}", result_two);

    println!("所有任务执行完成！");
}


/** Cargo.toml
[package]
name = "async_example"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
*/