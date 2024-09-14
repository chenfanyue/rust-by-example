在 Rust 中，错误传播与 `?` 运算符是处理错误的常用手段，它们可以简化代码中的错误处理逻辑。让我们从基础概念和具体使用开始，详细展开这个主题。

### 1. 错误传播的概念

Rust 中的错误处理使用 `Result` 枚举，它有两个变体：

- `Ok(T)`：表示成功，包含一个值 `T`。
- `Err(E)`：表示失败，包含一个错误值 `E`。

错误传播是指当函数调用可能产生错误时，我们可以将这个错误“传递”给调用者，让调用者决定如何处理错误，而不是在函数内部处理。

```rust
fn open_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}

fn main() {
    let content = open_file("file.path")
        .unwrap_or("failed".to_string());
    println!("{content}");
}
```
```rust
fn open_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}
fn main() {
    let content = open_file("src/file.path")
        .unwrap_or_else(|e| {
            println!("detailed error: {e}");
            String::from("failed text")
        });
    println!("{content}");
}
```

上面的例子展示了一个读取文件的函数，它返回一个 `Result<String, std::io::Error>`。如果读取成功，函数返回 `Ok(content)`；如果失败，它将返回一个 `Err`，其中包含错误信息。

### 2. `?` 运算符的作用

`?` 运算符用于简化错误处理流程。它背后的逻辑是：当 `?` 作用在 `Result` 上时，如果结果是 `Ok(T)`，程序会继续执行并提取出值 `T`；但如果是 `Err(E)`，函数会立即返回这个 `Err(E)`，并将错误传播给上一级调用者。

上面的例子中，`?` 的作用是让我们不用显式地编写匹配 `Result` 的逻辑，像这样：

```rust
fn open_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => return Err(e),
    };
    Ok(content)
}
```

使用 `?` 运算符，代码更加简洁明了。它帮助我们自动处理错误，而不需要手动解构 `Result` 枚举。

### 3. 错误传播与 `?` 运算符的限制

- **只能用于返回 `Result` 或 `Option` 类型的函数**：`?` 运算符要求调用的函数返回的也是一个 `Result` 或者 `Option` 类型。如果函数返回值不是这两者中的一种类型，则不能使用 `?` 运算符。
  
  例如，下面的代码会产生编译错误：

  ```rust
  fn example() {
      let result = std::fs::read_to_string("file.txt")?;
      // 错误：`?` 运算符只能用于返回 `Result` 或 `Option` 类型的函数
  }
  ```

  如果希望使用 `?` 运算符，函数的返回类型需要是 `Result` 或者 `Option`：

  ```rust
  fn example() -> Result<(), std::io::Error> {
      let result = std::fs::read_to_string("file.txt")?;
      Ok(())
  }
  ```

- **自动转换错误类型**：如果函数返回的错误类型和调用的函数不同，Rust 的 `?` 运算符可以结合 `From` trait 自动进行错误类型转换。

  ```rust
  use std::num::ParseIntError;

  fn parse_number(s: &str) -> Result<i32, ParseIntError> {
      let num: i32 = s.parse()?;
      Ok(num)
  }
  ```

  这里 `?` 自动将 `s.parse()` 中的错误类型 `ParseIntError` 传播给调用者。

### 4. 自定义错误类型和 `?` 运算符

在某些情况下，我们需要定义自己的错误类型。Rust 允许通过实现 `From` trait 来将不同的错误类型转换为我们定义的错误类型。

```rust
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum MyError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> MyError {
        MyError::ParseError(err)
    }
}

fn read_file_and_parse_number(file_path: &str) -> Result<i32, MyError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn main() {
    let n = read_file_and_parse_number("src/file.path")
        .unwrap_or(1001);
    println!("{n}");
}
fn main() {
    let n = match read_file_and_parse_number("src/file.path") {
        Ok(num) => num,
        Err(MyError::IoError(e)) => {
            println!("IO error: {e}");
            1001
        }
        Err(MyError::ParseError(e)) => {
            println!("Parse error: {e}");
            1001
        }
    };
    println!("{n}");
}
```

```rust
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum MyError {
    IoError,
    ParseError,
}

impl From<io::Error> for MyError {
    fn from(_err: io::Error) -> MyError {
        MyError::IoError
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(_err: std::num::ParseIntError) -> MyError {
        MyError::ParseError
    }
}

fn read_file_and_parse_number(file_path: &str) -> Result<i32, MyError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn main() {
    let n = read_file_and_parse_number("src/file.path")
        .unwrap_or(1001);
    println!("{n}");
}
```

在这个例子中，我们自定义了一个 `MyError` 枚举类型，并通过 `?` 运算符自动传播不同的错误类型（`io::Error` 和 `ParseIntError`），它们被自动转换为 `MyError` 类型。

### 5. 总结

- **`Result` 和 `Option` 是 Rust 错误处理的基础。**
- **`?` 运算符简化了错误传播，不需要手动解构 `Result` 或 `Option`。**
- **`?` 运算符要求函数返回 `Result` 或 `Option`，并且可以通过 `From` trait 进行错误类型的转换。**
- **自定义错误类型时，使用 `From` trait 可以让 `?` 运算符更灵活地处理不同类型的错误。**

这样，使用错误传播与 `?` 运算符可以让 Rust 的错误处理既安全又简洁
