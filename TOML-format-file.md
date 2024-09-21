### TOML 文件及其语法简介

**TOML** (Tom's Obvious, Minimal Language) 是一种简单易读的配置文件格式，它通过键值对来存储数据，常用于 Rust 项目的 `Cargo.toml` 文件。TOML 文件的设计目标是尽可能简单、易于解析且可读性强。

#### 1. **基本语法规则**
- **键值对**：TOML 的基本结构是键值对，每一行包含一个键和值，用等号`=`分隔。例如：

    ```toml
    name = "my_project"
    version = "0.1.0"
    ```

- **字符串**：用双引号包围，例如：

    ```toml
    description = "This is a Rust project."
    ```

- **整数和浮点数**：直接写数值即可，无需引号。例如：

    ```toml
    year = 2024
    pi = 3.1415
    ```

- **布尔值**：可以使用 `true` 或 `false`，例如：

    ```toml
    is_active = true
    ```

- **日期**：可以用 ISO 8601 格式表示日期和时间，例如：

    ```toml
    start_date = 2024-09-18T12:00:00Z
    ```

#### 2. **表 (Table)**
TOML 支持嵌套数据结构，使用表来分组相关的键值对。表名用方括号 `[ ]` 定义。例如：

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
```

#### 3. **嵌套表**
你可以在表中使用嵌套表来组织复杂数据结构。例如：

```toml
[dependencies]
serde = "1.0"

[dependencies.serde_derive]
version = "1.0"
features = ["derive"]
```

#### 4. **数组**
数组通过方括号 `[ ]` 表示，多个元素用逗号分隔。例如：

```toml
authors = ["Alice", "Bob"]
numbers = [1, 2, 3, 4, 5]
```

#### 5. **多行字符串**
当字符串太长时，可以使用三对双引号 `"""` 进行多行字符串定义。例如：

```toml
description = """
This is a long
description spanning multiple lines.
"""
```

### 常见应用场景

- **定义包的元数据**：如 `name`, `version`, `authors`。
- **管理依赖项**：如 `dependencies`，用于指定项目所依赖的其他库。
- **配置特性 (features)**：用于启用或禁用代码库中的某些功能模块。

TOML 作为 Rust 项目配置的主要文件格式，其简洁易读的设计非常适合用作配置文件，尤其是在像 `Cargo.toml` 这样的 Rust 项目配置文件中。
