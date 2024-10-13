在你的 Rust 1.80.1 学习大纲中，关于 **测试与调试** 章节的 **使用 `cargo test` 运行测试**，我可以为你展开如下内容：

---

### 测试与调试

#### 使用 `cargo test` 运行测试

`cargo test` 是 Rust 的标准工具，用于编译和运行测试。Rust 提供了一个强大的内置测试框架，允许开发者编写单元测试、集成测试，并使用各种断言来验证代码的行为。`cargo test` 会自动发现并运行项目中的所有测试。

##### 1. 基本用法

- **编写单元测试**：单元测试通常与被测试的代码放在同一个文件中，位于模块 `#[cfg(test)]` 下。编写测试函数时，需使用 `#[test]` 属性标记该函数为测试函数。

**示例**：
```rust
fn main() {
    println!("a pretty girl at the other side of the desk");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1000, 1001);
    }

    #[test]
    #[ignore]
    fn long_running_test() {
        // 这里是长时间运行的测试逻辑
    }
}
```

- **运行测试**：在终端运行以下命令：
  ```bash
  cargo test
  ```

  这将编译项目并执行所有带有 `#[test]` 属性的测试函数。测试通过时，会显示 `ok`，测试失败时，会显示详细的错误信息。

##### 2. `assert!`、`assert_eq!` 和 `assert_ne!` 断言

Rust 提供了几种常见的断言宏，帮助开发者验证测试的正确性：
- **`assert!`**：用于断言一个条件为 `true`。
  ```rust
  assert!(x > 0);
  ```
- **`assert_eq!`** 和 **`assert_ne!`**：分别用于检查两个值是否相等或不相等。
  ```rust
  assert_eq!(result, expected);
  assert_ne!(result, unexpected);
  ```

##### 3. 测试输出控制

- **隐藏通过的测试输出**：在默认情况下，`cargo test` 会显示所有通过和失败测试的输出。如果想隐藏通过的测试输出，可以运行：
  ```bash
  cargo test -- --quiet
  ```
  
- **仅运行指定测试**：可以通过指定测试名称来运行单个或一组测试。例如，运行名为 `it_works` 的测试：
  ```bash
  cargo test it_works
  ```

##### 4. 忽略测试 (`#[ignore]`)

如果某些测试需要较长时间运行，或者你暂时不希望它们运行，可以为其添加 `#[ignore]` 属性。通过以下方式运行被忽略的测试：
```bash
cargo test -- --ignored
```

**示例**：
```rust
#[test]
#[ignore]
fn long_running_test() {
    // 这里是长时间运行的测试逻辑
}
```

##### 5. 并行测试

默认情况下，`cargo test` 会并行运行测试。这可以加速测试运行时间，但在某些情况下，尤其是测试存在共享状态或需要顺序执行时，可能导致问题。可以通过 `--test-threads` 来控制测试并行的线程数量：
```bash
cargo test -- --test-threads=1
```

##### 6. 测试的分类

Rust 中的测试分为三类：

- **单元测试**：验证单个模块或功能的正确性，通常位于与源代码相同的文件中。
- **集成测试**：用于测试多个模块或组件的交互，位于项目的 `tests` 目录中。集成测试可以调用公共 API，但不能访问模块的私有内容。
  
##### 7. 失败测试的调试

在测试失败时，`cargo test` 会提供详细的堆栈跟踪信息，可以帮助你定位问题。如果想在调试模式下运行某个特定的测试，可以结合 `--nocapture` 选项来显示所有打印信息：
```bash
cargo test it_works -- --nocapture
```

##### 8. Benchmark 基准测试 (unstable)

Rust 还支持基准测试功能，但它目前属于不稳定特性，依赖于 `bench` 特性，并且需要在 `Cargo.toml` 中启用 nightly 版本的编译器：
```toml
[package]
...
[profile.bench]
debug = false
```

然后，你可以编写带有 `#[bench]` 属性的基准测试函数，并使用 `cargo bench` 来运行基准测试。

---

通过这个展开内容，你可以更深入了解如何使用 `cargo test` 进行有效的测试和调试。是否需要对某些部分再进一步解释或提供更多代码示例？