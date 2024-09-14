在 Rust 的错误处理机制中，`Result` 和 `Option` 是两个核心枚举类型，广泛用于处理可能的错误和可选值。以下是对这两者的详细介绍：

### 1. `Option` 枚举

`Option` 用于表示值的存在或缺失，它的定义如下：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

- **Some(T)**：包含了一个值 `T`，表示某个值是存在的。
- **None**：表示值缺失或不存在。

#### 用法
`Option` 常用于表示一个值可能为空的场景，如数组索引越界，或者查找操作。使用时可以配合模式匹配 (`match`) 或其他简化方法，如 `unwrap`、`unwrap_or`、`map`、`and_then` 等。

```rust
fn get_first_element(arr: &[i32]) -> Option<i32> {
    arr.get(0).copied()
}

let numbers = vec![1, 2, 3];
match get_first_element(&numbers) {
    Some(val) => println!("The first element is: {}", val),
    None => println!("Array is empty"),
}
```

#### 常用方法
- `unwrap()`：如果值为 `Some`，则返回内部的值；否则程序会崩溃。
- `unwrap_or(default)`：如果值为 `Some`，返回内部值，否则返回 `default`。
- `map()`：对 `Some` 内部的值进行操作，`None` 则直接返回。
  
### 2. `Result` 枚举

`Result` 用于表示操作的成功或失败，定义如下：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- **Ok(T)**：表示操作成功，包含返回的值 `T`。
- **Err(E)**：表示操作失败，包含错误信息 `E`。

#### 用法
`Result` 常用于可能失败的操作，例如文件读写、网络请求等。通过 `match` 语句或其他便捷方法处理 `Result` 的值。

```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err),
}
```

#### 常用方法
- `unwrap()`：成功时返回内部值 `T`，失败时程序崩溃。
- `expect(msg)`：类似 `unwrap()`，但可以自定义错误信息。
- `unwrap_or(default)`：失败时返回默认值。
- `and_then()`：链式操作，只有当 `Result` 为 `Ok` 时才执行。

### 3. `Option` 与 `Result` 的区别
- **Option**：用于表示值是否存在，仅有两种状态 `Some` 和 `None`。
- **Result**：用于处理错误，带有 `Ok`（成功）和 `Err`（失败）两种状态。

### 错误传播

Rust 提供了 `?` 操作符来简化错误处理和传播。当调用返回 `Result` 或 `Option` 类型的函数时，可以使用 `?` 将错误自动传播给调用者。

```rust
fn read_file() -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

这种方式减少了大量的 `match` 语句，使代码更加简洁。
