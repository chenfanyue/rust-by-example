Rust 支持 **字符串插值**，通过多种方式将变量、表达式等插入到字符串中。主要有两种常见方式进行字符串插值：

1. 使用 `format!` 宏。
2. 使用 `println!` 等输出宏。

下面我们详细系统地介绍这些插值方式。

### 1. `format!` 宏

`format!` 宏用于格式化字符串，它不会输出到控制台，而是返回一个 `String` 类型的值。可以通过 `{}` 等占位符将变量插入字符串中，类似其他语言的字符串插值功能。

#### 基本用法：
```rust
let name = "Alice";
let age = 30;
let formatted_string = format!("Hello, my name is {} and I am {} years old.", name, age);
println!("{}", formatted_string);
```

#### 输出：
```
Hello, my name is Alice and I am 30 years old.
```

在 `format!` 中，`{}` 占位符用于插入变量。

#### 复杂的格式控制：
可以使用格式化字符串控制输出的宽度、对齐和精度等。例如，使用 `{:width}` 控制宽度，`{:.precision}` 控制小数点后的精度。

```rust
let pi = 3.14159;
let formatted = format!("Pi to three decimal places is {:.3}", pi);
println!("{}", formatted);
```

#### 输出：
```
Pi to three decimal places is 3.142
```

#### 其他格式选项：
- `{:?}` 用于格式化调试输出（即 `Debug` trait），例如打印结构体或枚举的调试信息。
- `{:#?}` 会以更易读的方式格式化调试输出，特别适合多行结构体或枚举。
- `{value:x}` 指定按 16 进制格式化数字，类似地 `{value:o}` 为八进制，`{value:b}` 为二进制。

#### 示例：
```rust
let value = 255;
println!("Decimal: {}, Hex: {:x}, Binary: {:b}", value, value, value);
```

#### 输出：
```
Decimal: 255, Hex: ff, Binary: 11111111
```

### 2. `println!` 宏

`println!` 是 `format!` 的一个特殊版本，它会将格式化后的字符串输出到标准输出。用法上类似于 `format!`，但是不返回 `String`，而是直接打印。

#### 示例：
```rust
let x = 42;
let y = 3.14159;
println!("x = {}, y = {:.2}", x, y);
```

#### 输出：
```
x = 42, y = 3.14
```

### 3. 使用变量名作为占位符：`named parameters`

Rust 不直接支持像 Python f-string 那样的直接变量插值（`f"Hello {name}"`），但你可以在 `format!` 中通过索引或命名参数实现类似效果。

#### 使用索引：
```rust
let name = "Alice";
let age = 30;
let formatted_string = format!("{0} is {1} years old. {0} likes Rust.", name, age);
println!("{}", formatted_string);
```

#### 输出：
```
Alice is 30 years old. Alice likes Rust.
```

#### 使用命名参数：
```rust
let name = "Alice";
let age = 30;
let formatted_string = format!("{name} is {age} years old.", name=name, age=age);
println!("{}", formatted_string);
```

#### 输出：
```
Alice is 30 years old.
```

### 4. `to_string()` 和 `String::from()`

虽然这些不是严格意义上的插值操作，但在 Rust 中，许多类型实现了 `ToString` trait，允许它们通过 `to_string()` 方法直接转换为 `String`，有助于插值操作时进行类型转换。

#### 示例：
```rust
let x = 10;
let y = 3.14;
let s = x.to_string() + " and " + &y.to_string();
println!("{}", s);
```

#### 输出：
```
10 and 3.14
```

### 5. 常见的格式化类型说明

| 格式      | 说明                                  | 示例输出 |
| --------- | ------------------------------------- | -------- |
| `{}`      | 默认显示，调用 `Display` trait          | `42`     |
| `{:?}`    | 调试显示，调用 `Debug` trait            | `[1, 2, 3]` |
| `{:#?}`   | 以美化方式进行调试显示                 | `[\n 1,\n 2,\n 3,\n]` |
| `{:x}`    | 以小写十六进制显示                    | `2a`     |
| `{:X}`    | 以大写十六进制显示                    | `2A`     |
| `{:.2}`   | 精度为 2 的浮点数                     | `3.14`   |
| `{:<5}`   | 左对齐，宽度为 5                      | `42   `  |
| `{:>5}`   | 右对齐，宽度为 5                      | `   42`  |
| `{:^5}`   | 居中对齐，宽度为 5                    | ` 42  `  |
| `{:.3}`   | 浮点数保留 3 位小数                  | `3.142`  |

### 6. 格式化结构体和枚举

对于自定义类型，必须手动实现 `Display` 或 `Debug` trait 才能使用 `format!` 或 `println!`。对于调试输出，可以自动派生 `Debug` trait。

#### 示例：
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };
    println!("{:?}", point);  // 调试输出
    println!("{:#?}", point);  // 美化的调试输出
}
```

#### 输出：
```
Point { x: 10, y: 20 }
Point {
    x: 10,
    y: 20,
}
```

### 总结

- **`format!`** 是 Rust 中进行字符串插值的主要方式，通过 `{}` 占位符进行插值。
- **`println!`** 是 `format!` 的特殊版本，用于标准输出。
- **命名参数** 和 **索引** 可以帮助实现更灵活的插值方式。
- **格式化选项**（如宽度、对齐、精度）可以控制插值的显示格式。
- **自定义类型** 需要手动实现 `Display` 或 `Debug` trait 才能进行格式化插值。

通过这些机制，Rust 提供了强大的字符串插值功能，既高效又灵活。

