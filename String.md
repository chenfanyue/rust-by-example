在 Rust 中，`String` 是一个可增长的、可变的 UTF-8 编码字符串类型，主要用于存储动态的文本数据。它在堆上分配内存，并且可以随着需要动态地扩展。`String` 是 Rust 标准库中的一个非常常用的类型，具备许多实用的方法。

### 1. `String` vs `&str`

- `String`：是一个拥有所有权的、可变的字符串类型，存储在堆上。它可以随着内容的改变而增长。
- `&str`：是一个不可变的字符串切片，它只是对一段 UTF-8 编码文本的引用，通常存储在栈上或程序的二进制中。

#### 示例：

```rust
let s1 = String::from("Hello");
let s2 = "World"; // 这是一个 &str
```

在这里，`s1` 是一个拥有所有权的 `String`，而 `s2` 是一个字符串切片 `&str`，不可变且不可增长。

---

### 2. 创建 `String`

- `String::new()`：创建一个空的 `String`。
- `String::from()`：从一个字符串切片 `&str` 创建一个 `String`。

#### 示例：

```rust
let empty = String::new(); // 空的 String
let s = String::from("Hello, world!"); // 从 &str 创建 String
```

- `to_string()`：可以将 `&str` 转换为 `String`。

```rust
let s = "Hello".to_string();
```

---

### 3. `String` 的基本方法

#### 1) `push()`
`push` 方法可以向 `String` 末尾追加一个字符。

```rust
let mut s = String::from("Hello");
s.push('!'); // 追加字符
println!("{}", s); // 输出: Hello!
```

#### 2) `push_str()`
`push_str` 方法可以向 `String` 末尾追加一个字符串切片。

```rust
let mut s = String::from("Hello");
s.push_str(", world");
println!("{}", s); // 输出: Hello, world
```

#### 3) `+` 运算符
可以使用 `+` 运算符将两个 `String` 或 `&str` 拼接在一起，但需要注意的是，`+` 运算符的第一个操作数必须是 `String` 类型，且它会移动（消耗）该 `String`，而不是复制。

```rust
let s1 = String::from("Hello");
let s2 = String::from("world");
let s3 = s1 + " " + &s2;
println!("{}", s3); // 输出: Hello world
// 注意：s1 被移动了，不能再使用 s1
```

#### 4) `format!()`
`format!` 宏提供了一种更加方便且不会消耗原始 `String` 的拼接方式。

```rust
let s1 = String::from("Hello");
let s2 = String::from("world");
let s3 = format!("{} {}", s1, s2);
println!("{}", s3); // 输出: Hello world
// 注意：s1 和 s2 仍然可用
```

#### 5) `len()`
`len` 方法返回 `String` 的字节长度，而不是字符数。因为 UTF-8 编码的字符可能占多个字节，所以字符数与字节数可能不相等。

```rust
let s = String::from("你好");
println!("{}", s.len()); // 输出 6，因为 "你" 和 "好" 各占 3 个字节
```

#### 6) `capacity()`
`capacity` 方法返回 `String` 当前分配的容量（字节数），即可以在不重新分配内存的情况下存储的字节数。

```rust
let mut s = String::with_capacity(10);
println!("{}", s.capacity()); // 输出 10
s.push_str("Hello");
println!("{}", s.capacity()); // 输出 10 (未重新分配)
```

#### 7) `clear()`
`clear` 方法会清空 `String` 的内容，但不会改变它的容量。

```rust
let mut s = String::from("Hello");
s.clear();
println!("{}", s); // 输出: 空字符串
```

---

### 4. `String` 索引与切片

Rust 不允许通过索引直接访问 `String` 的字符，这是因为 `String` 是 UTF-8 编码的，字符可能占多个字节。如果需要访问特定字符，应该使用切片。

```rust
let s = String::from("Hello, world!");
let slice = &s[0..5]; // 创建字符串切片
println!("{}", slice); // 输出: Hello
```

#### 注意：避免无效切片
在 Rust 中，必须保证字符串切片总是有效的 UTF-8 编码边界，否则会引发运行时错误。例如，以下代码会崩溃，因为 `&s[0..1]` 无法切出一个有效的 UTF-8 字符。

```rust
let s = String::from("你好");
let slice = &s[0..1]; // 错误：无效的切片边界
```

---

### 5. 遍历 `String`

可以通过多种方式遍历 `String`，包括按字符、按字节或按 Unicode 标量值遍历。

#### 1) 按字符遍历
```rust
let s = String::from("Hello");
for c in s.chars() {
    println!("{}", c);
}
```

#### 2) 按字节遍历
```rust
let s = String::from("Hello");
for b in s.bytes() {
    println!("{}", b);
}
```

---

### 6. `String` 的常用方法

#### 1) `contains()`
检查字符串是否包含指定的子字符串。

```rust
let s = String::from("Hello, world!");
println!("{}", s.contains("world")); // 输出: true
```

#### 2) `find()`
返回第一个匹配子字符串的字节索引。

```rust
let s = String::from("Hello, world!");
if let Some(index) = s.find("world") {
    println!("Found at index: {}", index); // 输出: Found at index: 7
}
```

#### 3) `replace()`
将字符串中的某个子字符串替换为另一个字符串。

```rust
let s = String::from("I like cats");
let new_s = s.replace("cats", "dogs");
println!("{}", new_s); // 输出: I like dogs
```

#### 4) `split()`
根据分隔符将字符串拆分成多个部分。

```rust
let s = String::from("Hello, world!");
for part in s.split(", ") {
    println!("{}", part); // 输出: Hello 和 world!
}
```

#### 5) `trim()`
移除字符串两端的空白字符。

```rust
let s = String::from("  Hello  ");
println!("{}", s.trim()); // 输出: Hello
```

#### 6) `is_empty()`
检查字符串是否为空。

```rust
let s = String::new();
println!("{}", s.is_empty()); // 输出: true
```

---

### 7. `String` 的内存结构

- `String` 的数据存储在**堆**上，内部包含三部分：指向堆中实际数据的指针、存储字符串长度的字段、以及容量字段（表示可以存储的最大字节数）。
- `String` 可以随着字符串的增长或缩小而动态分配或释放内存，Rust 的所有权系统会确保这些内存管理操作是安全的。

---

### 总结

`String` 是 Rust 中一个功能强大且高效的字符串类型，它支持动态扩展、拥有灵活的操作方法，并且与 `&str` 紧密配合使用。通过了解 `String` 的操作方法和背后的内存管理机制，可以更加高效地处理动态文本数据。
