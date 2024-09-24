### 4. 并发编程

#### 线程与消息传递

##### 消息传递与通道 (`std::sync::mpsc`)

**概述：**  
Rust 标准库提供的 `std::sync::mpsc` 模块允许线程之间通过通道（channel）进行消息传递。`mpsc` 代表“multiple producer, single consumer”（多生产者，单消费者），即多个线程可以发送消息到一个接收者。Rust 的通道实现基于类型安全和所有权系统，确保消息传递中的数据竞态条件得到有效管理。

**1. 创建通道**

Rust 提供的通道由两个部分组成：发送者（sender）和接收者（receiver）。使用 `mpsc::channel()` 函数可以创建一个通道：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("Hello");
        tx.send(val).unwrap(); // 通过发送者发送消息
    });

    let received = rx.recv().unwrap(); // 通过接收者接收消息
    println!("Got: {}", received);
}
```

在此例中，`mpsc::channel()` 返回一对 `(tx, rx)`，其中 `tx` 是发送者，`rx` 是接收者。一个线程使用 `tx.send()` 发送消息，而另一个线程使用 `rx.recv()` 接收消息。注意，发送者通过 `move` 关键字传递了消息的所有权。

**2. 多生产者**

通过克隆发送者可以实现多生产者场景。多个发送者可以同时向同一个接收者发送消息：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("Hello from thread 1")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("Hello from thread 2")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

这里，`tx.clone()` 创建了第二个发送者 `tx1`。这两个线程都能发送消息，接收者可以接收到来自不同生产者的消息。

**3. 非阻塞接收**

默认情况下，`recv()` 是阻塞操作，会等待消息到达。然而，可以使用 `try_recv()` 来非阻塞地尝试接收消息，如果通道中没有消息，它会立即返回一个错误，而不会阻塞：

```rust
match rx.try_recv() {
    Ok(message) => println!("Received: {}", message),
    Err(_) => println!("No messages available"),
}
```

`try_recv()` 常用于需要在不阻塞主线程的情况下处理其他任务的场景。

**4. 发送者和接收者的所有权**

Rust 的所有权系统确保了消息在通过通道时的安全性。一旦消息被发送者传递给接收者，发送者就不再拥有消息的所有权。因此，发送的消息无法再被发送方访问，避免了数据竞态条件。

**5. 通道的关闭**

当所有发送者都被丢弃时，通道将自动关闭，此时接收者会收到一个错误 `Err(TryRecvError::Disconnected)`，表明通道已被断开。这可以用于在消息传递结束时通知接收者。

**6. 示例：用于并行任务处理**

通道非常适合用于在多线程应用中处理任务或传递结果。以下是一个线程池使用通道来调度和处理任务的简单例子：

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            tx.send(i).unwrap();
        });
    }

    for received in rx {
        println!("Got: {}", received);
    }
}
```

在这个例子中，五个线程并行执行任务，每个线程在完成任务后通过通道发送结果，主线程通过接收者获取所有结果。

---

**总结：**  
- `mpsc::channel()` 创建一个多生产者单消费者通道。
- 发送者使用 `send()` 发送消息，接收者使用 `recv()` 或 `try_recv()` 接收消息。
- 通过克隆发送者可以实现多生产者场景。
- Rust 的所有权和类型系统确保了消息传递的安全性，避免数据竞态问题。

这为 Rust 提供了一种安全且高效的线程间通信方式，尤其适用于需要并发执行任务的场景。
