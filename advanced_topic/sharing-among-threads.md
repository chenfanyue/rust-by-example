```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let lock = Arc::new(RwLock::new(5));
    let mut handles = vec![];

    // 创建多个线程同时读取数据
    for _ in 0..10 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            let r = lock.read().unwrap();
            println!("Read: {}", *r);
        });
        handles.push(handle);
    }

    // 创建一个线程写入数据
    let lock = Arc::clone(&lock);
    let handle = thread::spawn(move || {
        let mut w = lock.write().unwrap();
        *w += 1;
        println!("Write: {}", *w);
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
```

我又仔细研读了一下完整的代码，发现精髓其实在这2句
let rwlock = Arc::clone(&rwlock);
let cv = Arc::clone(&cv);

先把 mutex 和 condvar 打包成元组，再放到高层的 arc  智能指针数据结构里，然后在每个线程中通过对 arc 的引用间接实现了对 mutex 和 condvar 的共享
mutex 是线程安全的，condvar 即便本身不是线程安全的，但在每个线程内部都和 mutex 同进同退，也就相当于具备了线程安全的特性

只能说创造这个代码的人简直是天才呀