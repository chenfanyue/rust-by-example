use async_std::task;
use std::time::Duration;

fn main() {
    task::block_on(async {
        let task1 = task::sleep(Duration::from_secs(1));
        let task2 = task::sleep(Duration::from_secs(2));
        task1.await;
        println!("Task 1 done");
        task2.await;
        println!("Task 2 done");
    });
}



use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

fn main() -> std::io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        while let Some(stream) = listener.incoming().next().await {
            let mut stream = stream?;
            task::spawn(async move {
                let mut buffer = [0; 1024];
                let n = stream.read(&mut buffer).await.expect("Failed to read data");
                stream
                    .write_all(&buffer[0..n])
                    .await
                    .expect("Failed to write data");
            });
        }
        Ok(())
    })
}

