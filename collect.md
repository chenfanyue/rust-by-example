```rust
fn main() {
    let a = [1, 2, 3];

    let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();

    assert_eq!(vec![2, 4, 6], doubled);
}


use std::collections::VecDeque;

fn main() {
    let a = [1, 2, 3];

    let doubled: VecDeque<i32> = a.iter().map(|&x| x * 2).collect();

    assert_eq!(2, doubled[0]);
    assert_eq!(4, doubled[1]);
    assert_eq!(6, doubled[2]);
}
```

`collect` 是 Rust 中非常强大且常用的方法之一，它适用于将迭代器的元素转换为集合类型。`collect` 不仅可以将迭代器转换为 `Vec`，还可以转换为其他集合类型，例如 `HashMap`。它通过类型推导自动选择适当的目标集合类型，因此使用时需要显式指定结果类型，或者通过上下文来推断。

### 用于 `HashMap` 的 `collect` 方法
我们可以将 `(K, V)` 元组的迭代器收集为 `HashMap`。这种用法在处理键值对数据时特别有用。

#### 示例 1：将 `Vec` 转换为 `HashMap`
通过 `collect`，可以将一个 `(K, V)` 的 `Vec` 转换为 `HashMap`：

```rust
use std::collections::HashMap;

fn main() {
    let pairs = vec![
        ("苹果", 3),
        ("香蕉", 2),
        ("橙子", 5),
    ];

    // 将 Vec<(K, V)> 转换为 HashMap<K, V>
    let map: HashMap<_, _> = pairs.into_iter().collect();

    println!("{:?}", map); // 输出: {"苹果": 3, "香蕉": 2, "橙子": 5}
}
```

在这个例子中，`Vec<(K, V)>` 被转换为 `HashMap<K, V>`，其中 `_` 是自动推断的键和值的类型。

#### 示例 2：通过迭代器生成键值对并转换为 `HashMap`
可以通过创建迭代器来生成键值对，并使用 `collect` 将其转换为 `HashMap`。

```rust
use std::collections::HashMap;

fn main() {
    // 使用 iter 和 zip 生成键值对
    let keys = vec!["苹果", "香蕉", "橙子"];
    let values = vec![3, 2, 5];

    let map: HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();

    println!("{:?}", map); // 输出: {"苹果": 3, "香蕉": 2, "橙子": 5}

    let enumerate: Vec<_> = "foo".chars().enumerate().collect();
    let zipper: Vec<_> = (0..).zip("foo".chars()).collect();

    assert_eq!((0, 'f'), enumerate[0]);
    assert_eq!((0, 'f'), zipper[0]);

    assert_eq!((1, 'o'), enumerate[1]);
    assert_eq!((1, 'o'), zipper[1]);

    assert_eq!((2, 'o'), enumerate[2]);
    assert_eq!((2, 'o'), zipper[2]);
}
```

在这个例子中，`zip` 方法将两个向量的元素配对，并生成键值对迭代器，最终使用 `collect` 转换为 `HashMap`。

### `collect` 的适用场景
1. **从迭代器生成集合**：`collect` 可以从迭代器中快速生成各种集合，包括 `HashMap`、`Vec`、`HashSet` 等。
2. **批量处理**：通过迭代器生成键值对，使用 `collect` 可以快速构建哈希映射，减少手动插入的代码量。

### 总结
`collect` 方法在 Rust 中提供了一种简洁且强大的方式将迭代器的内容转换为集合类型，尤其是在处理键值对数据时，它可以非常方便地生成 `HashMap`。通过适当的类型推断和显式类型指定，`collect` 提供了灵活的集合构建能力。