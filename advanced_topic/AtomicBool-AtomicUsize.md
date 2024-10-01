在 Rust 1.80.1 的学习大纲中，关于“同步与互斥”部分的原子类型 (`AtomicBool` 和 `AtomicUsize`) 可以详细展开为以下内容：

### 1. 原子类型概述
Rust 提供了多种原子类型用于保证线程安全的操作，而无需锁机制。原子操作是不可分割的，它确保对变量的读取-修改-写入操作在多个线程之间不会产生竞态条件（Race Condition）。

**常见原子类型：**
- `AtomicBool`: 表示一个原子的布尔值，支持基本的原子操作。
- `AtomicUsize`: 表示一个原子的无符号整数，通常与系统字长有关，用于原子计数等场景。

### 2. `AtomicBool` 详解
`AtomicBool` 是一个用来在线程间共享布尔值的类型，它提供了许多方法以原子方式对布尔值进行操作：

**常见方法：**
- `new(value: bool) -> AtomicBool`: 创建一个新的原子布尔值。
- `load(Ordering) -> bool`: 以指定的内存顺序读取值。
- `store(val: bool, Ordering)`: 以指定的内存顺序存储值。
- `swap(val: bool, Ordering) -> bool`: 将当前值替换为 `val` 并返回旧值。
- `compare_and_swap(current: bool, new: bool, Ordering) -> bool`: 如果当前值等于 `current`，则将其设为 `new`，并返回旧值。

**代码示例：**
```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let flag = Arc::new(AtomicBool::new(true));
    let flag_clone = Arc::clone(&flag); // 克隆 Arc 以在另一个线程中共享所有权

    let handle = thread::spawn(move || {
        flag_clone.store(false, Ordering::SeqCst); // 将 flag 设置为 false
    });

    handle.join().unwrap();

    // 这里的 flag 仍然有效，因为我们使用 Arc 来共享所有权
    println!("Flag value: {}", flag.load(Ordering::SeqCst)); // 读取 flag 的值
}
```

**场景应用：**
- 原子布尔值可以用来实现简单的开关机制，决定是否允许某个操作执行。
- 在多个线程中可以安全地读取和修改布尔值，而不会引发数据竞争。

### 3. `AtomicUsize` 详解
`AtomicUsize` 是一个用来处理无符号整数的原子类型，常用于计数器、索引等场景。它通常与系统字长一致（例如在 64 位系统上为 64 位）。

**常见方法：**
- `new(value: usize) -> AtomicUsize`: 创建一个新的原子无符号整数。
- `load(Ordering) -> usize`: 以指定的内存顺序读取值。
- `store(val: usize, Ordering)`: 以指定的内存顺序存储值。
- `fetch_add(val: usize, Ordering) -> usize`: 原子地增加值，并返回之前的值。
- `fetch_sub(val: usize, Ordering) -> usize`: 原子地减少值，并返回之前的值。
- `compare_and_swap(current: usize, new: usize, Ordering) -> usize`: 如果当前值等于 `current`，则将其设为 `new`，并返回旧值。

**代码示例：**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter_clone = Arc::clone(&counter); // 克隆 Arc 以共享所有权
            thread::spawn(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst); // 原子地增加计数器
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // 这里的 counter 仍然有效，因为我们使用 Arc 来共享所有权
    println!("Counter: {}", counter.load(Ordering::SeqCst)); // 输出计数器的最终值
}
```

**场景应用：**
- `AtomicUsize` 常用于实现计数器或索引，在多线程环境下保证计数操作的原子性。
- 可以用于追踪多线程任务的完成情况，避免手动使用锁进行同步。

### 4. 内存排序 (Ordering)
`AtomicBool` 和 `AtomicUsize` 的方法通常需要指定一个内存排序（`Ordering`）。内存排序决定了读写操作在不同线程间的可见性。常见的内存排序包括：
- `Ordering::Relaxed`: 最弱的排序，不保证其他线程立刻看到更新。
- `Ordering::Acquire`: 保证此读操作及其后续操作不会在内存中与之前的操作重排。
- `Ordering::Release`: 保证此写操作及其前面的操作不会在内存中与之后的操作重排。
- `Ordering::SeqCst`: 最强的排序，保证全局一致的顺序。
`Ordering` 是 Rust 中用于控制原子操作的内存顺序，它定义了读写操作在多线程环境中的可见性和重排序规则。不同的 `Ordering` 选项会影响原子操作如何在 CPU 内部被处理，尤其是在多核处理器或多线程环境中。Rust 提供了以下几种常见的内存顺序：

### 1. 内存排序（Memory Ordering）概述
在现代处理器中，指令可能会因为性能优化的原因被重新排列。这个重排序在单线程程序中不会影响最终结果，但在多线程程序中，如果不控制好内存访问的顺序，可能会引发数据竞态（Race Condition）等问题。

内存排序用于告诉编译器和 CPU 何时可以重排序操作，何时不能重排序，以确保不同线程间的数据读写操作不会产生不一致。

### 2. 常见的 `Ordering` 类型
Rust 中的原子操作通常需要传递一个 `Ordering` 参数，这个参数定义了操作的内存排序规则。

#### 2.1 `Ordering::Relaxed`
`Relaxed` 是最弱的内存排序，不对操作的可见性或重排序进行额外限制。它只保证操作本身是原子的，也就是说多个线程不会同时访问和修改数据，但不保证其他线程能立即看到最新的值。

**特点：**
- 保证原子操作的正确性（不会引发竞态条件）。
- 不保证操作的执行顺序或可见性。
- 适用于不关心操作顺序的场景，例如计数器。

**例子：**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);
counter.fetch_add(1, Ordering::Relaxed); // 不关心操作顺序
```

#### 2.2 `Ordering::Acquire` 和 `Ordering::Release`
`Acquire` 和 `Release` 是成对出现的，用于保证内存操作的顺序：

- `Acquire` 用于读取操作，保证当前线程在读取数据后不会重排它之前的操作。也就是说，当前线程读取的数据是其他线程在 `Release` 操作之后写入的结果。
- `Release` 用于写入操作，确保当前线程在写入数据前不会重排之后的操作。也就是说，当前线程写入的数据对其他线程的 `Acquire` 操作可见。

**特点：**
- 保证跨线程的顺序一致性，适合使用锁或信号量等同步机制的场景。
- 用于跨线程的生产者-消费者模式：生产者使用 `Release` 写入数据，消费者使用 `Acquire` 读取数据，确保消费者可以看到生产者发布的所有内存状态变化。

**例子：**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

let data = AtomicUsize::new(0);

// 在一个线程中设置值
let handle = thread::spawn(move || {
    data.store(42, Ordering::Release); // 保证写入操作在发布之前不会被重排
});

// 在另一个线程中读取值
handle.join().unwrap();
println!("{}", data.load(Ordering::Acquire)); // 保证读取操作不会在发布前被重排
```

#### 2.3 `Ordering::SeqCst`
`SeqCst` 是最强的内存顺序，表示**全局顺序一致性**。它保证了所有原子操作（在所有线程中）都按严格的顺序执行，不允许重排。

**特点：**
- 强制所有线程看到的操作顺序是一致的。
- 性能开销较大，适用于要求强一致性和顺序的场景。

**例子：**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);
counter.fetch_add(1, Ordering::SeqCst); // 所有线程的操作顺序是一致的
```

### 3. 选择合适的 `Ordering`
选择 `Ordering` 时需要权衡性能和数据的一致性：
- **`Relaxed`**：用于不关心操作顺序的场景，例如计数器更新。
- **`Acquire`/`Release`**：用于跨线程的数据同步，确保操作之间有合理的顺序，例如生产者-消费者模型。
- **`SeqCst`**：用于需要强一致性、全局顺序的场景，但性能相对较差。

**总结：**  
- `Relaxed` 是最弱的，它只保证操作的原子性，但不保证操作顺序。
- `Acquire` 和 `Release` 用于控制跨线程的同步，确保操作按一定顺序执行。
- `SeqCst` 强制全局顺序一致性，是最严格的选择。

根据不同的应用场景，你可以选择合适的内存排序来在多线程环境中进行安全、正确的原子操作。

### 5. 性能和使用场景
原子操作不需要锁，因此相比使用 `Mutex` 等同步原语，它们在某些场景下性能更高。但由于它们只适用于简单的共享状态修改，如计数器、标志位等，它们不适合复杂数据结构的共享修改。

**适用场景：**
- 多线程的计数操作：如实现一个线程安全的计数器。
- 状态标记：如在线程池中标记某个任务是否完成。

通过这些扩展内容，你可以在 Rust 的原子类型部分更深入地学习和实践如何利用原子操作来构建高效且线程安全的并发程序。