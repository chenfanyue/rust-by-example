在 Rust 中，`HashMap` 是一种键值对（key-value）的集合，提供了快速的查找、插入和删除操作。它是基于哈希表实现的，键是唯一的，而值可以是任何类型。`HashMap` 是 Rust 标准库中的常用集合类型之一。接下来我会详细介绍 `HashMap` 的各种常用方法，并提供相应的代码示例。

### 1. 创建 `HashMap`
`HashMap` 可以通过 `HashMap::new()` 创建一个新的空的哈希映射，或者通过 `HashMap::from()` 使用一系列键值对初始化。

```rust
use std::collections::HashMap;

fn main() {
    // 创建一个空的 HashMap
    let mut map = HashMap::new();

    // 插入键值对
    map.insert("苹果", 3);
    map.insert("香蕉", 2);

    // 打印 HashMap
    println!("{:?}", map);
}


use std::collections::HashMap;

fn main() {
    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
    println!("{:#?}", solar_distance);
}


use std::collections::HashMap;

fn main() {
    let map1 = HashMap::from([(1, 2), (3, 4)]);
    let map2: HashMap<_, _> = [(1, 2), (3, 4)].into();
    assert_eq!(map1, map2);
}
```

### 2. 插入值：`insert`
`insert` 方法允许向 `HashMap` 中添加新的键值对。如果键已经存在，则会覆盖之前的值。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("橙子", 5);
    
    // 如果键已经存在，值会被替换
    map.insert("橙子", 10);

    println!("{:?}", map); // 输出：{"橙子": 10}
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    assert_eq!(map.insert(1001, "a"), None);
    assert_eq!(map.is_empty(), false);

    assert_eq!(map.insert(1001, "b"), Some("a"));
    assert_eq!(map[&1001], "b");
}
```

### 3. 访问值：`get`
`get` 方法通过键来获取对应的值。返回一个 `Option<&V>`，如果键存在则返回 `Some(&值)`，如果不存在则返回 `None`。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("苹果", 4);

    // 使用 `get` 访问值
    if let Some(&value) = map.get("苹果") {
        println!("苹果的数量是: {}", value);
    } else {
        println!("苹果不存在");
    }
}

use std::collections::HashMap;

fn main() {
    let mut solar_distance: HashMap<_, _> = [
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ].into();
    if let Some(v_ref) = solar_distance.get_mut("Earth") {
        println!("{:?}", v_ref);
        *v_ref += 2.0;
    }
    println!("{:?}", solar_distance);
}

use std::collections::HashMap;

fn main() {
    let solar_distance: HashMap<_, _> = [
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ].into();
    if let Some((&k, &v)) = solar_distance.get_key_value("Earth") {
        println!("{}: {}", k, v);
    }
    println!("{:?}", solar_distance);
}

// 通过索引引用直接拿到value值
use std::collections::HashMap;

fn main() {
    let solar_distance: HashMap<_, _> = [
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ].into();
    println!("{}", solar_distance["Earth"]);
}
```

### 4. 检查键：`contains_key`
`contains_key` 用于检查某个键是否存在于 `HashMap` 中，返回一个布尔值。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("香蕉", 2);

    if map.contains_key("香蕉") {
        println!("香蕉存在");
    } else {
        println!("香蕉不存在");
    }
}
```

### 5. 删除键：`remove`
`remove` 方法可以从 `HashMap` 中删除指定的键，并返回与该键关联的值。如果键不存在，则返回 `None`。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("葡萄", 6);

    // 删除键
    let removed = map.remove("葡萄");
    
    match removed {
        Some(val) => println!("删除的值是: {}", val),
        None => println!("该键不存在"),
    }
}

use std::collections::HashMap;

let mut map = HashMap::new();
map. insert(1, "a");
assert_eq!(map. remove(&1), Some("a"));
assert_eq!(map. remove(&1), None);
```

### 6. 迭代 `HashMap`
可以通过 `for` 循环迭代 `HashMap`，访问其中的键和值。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("苹果", 3);
    map.insert("香蕉", 2);
    map.insert("橙子", 4);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("苹果", 3);
    map.insert("香蕉", 2);
    map.insert("橙子", 4);

    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }
    println!("{:?}",map);
}
```

### 7. 获取键和值的所有权：`into_iter`
`into_iter` 会消耗 `HashMap`，返回所有键和值的所有权。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("苹果", 3);
    map.insert("香蕉", 2);

    for (key, value) in map.into_iter() {
        println!("{}: {}", key, value);
    }
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("苹果", 3);
    map.insert("香蕉", 2);

    for (key, value) in map {
        println!("{}: {}", key, value);
    }
}
```

### 8. 条件插入：`entry` 和 `or_insert`
`Entry` 枚举是 Rust 标准库中的一个枚举类型，用于在操作 `HashMap` 时提供灵活的键值访问。`Entry` 枚举的主要作用是允许我们在 `HashMap` 中安全地处理键的存在性问题——即在键存在时获取对应的值，而在键不存在时插入新值。`Entry` 枚举有两个变体：

### 1. `Occupied`
`Occupied` 变体表示键在 `HashMap` 中已经存在。通过它，可以访问和修改现有的值。

#### 示例：
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("苹果", 3);

    match map.entry("苹果") {
        std::collections::hash_map::Entry::Occupied(mut entry) => {
            // 键 "苹果" 已经存在
            let value = entry.get();
            println!("苹果的值是: {}", value);

            // 修改现有的值
            entry.insert(5);
            println!("修改后的苹果的值是: {}", entry.get());
        }
        std::collections::hash_map::Entry::Vacant(_) => {
            // 不会进入此分支
        }
    }
}
```

### 2. `Vacant`
`Vacant` 变体表示键在 `HashMap` 中不存在。可以通过它插入新的值。

#### 示例：
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    match map.entry("香蕉") {
        std::collections::hash_map::Entry::Occupied(_) => {
            // 不会进入此分支
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            // 键 "香蕉" 不存在，插入新值
            entry.insert(2);
        }
    }

    println!("{:?}", map); // 输出: {"香蕉": 2}
}
```

### `Entry` 枚举的变体：
- **`Occupied`**: 键已存在，提供访问和修改已有值的能力。
- **`Vacant`**: 键不存在，允许插入新的键值对。
这种设计为条件插入和安全访问提供了强大的功能，并且避免了不必要的重复查找操作。

`entry` 方法返回一个枚举 `Entry`，允许你检查某个键是否存在，并根据结果执行不同的操作。`or_insert` 会在键不存在时插入默认值。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("苹果", 3);

    // 如果 "香蕉" 存在则返回值，不存在则插入 2
    map.entry("香蕉").or_insert(2);
    map.entry("苹果").or_insert(10); // 不会插入，因为 "苹果" 已经存在

    println!("{:?}", map); // 输出: {"苹果": 3, "香蕉": 2}
}

use std::collections::HashMap;

let mut letters = HashMap::new();

for ch in "a short treatise on fungi".chars() {
    letters
        .entry(ch)
        .and_modify(|counter| *counter += 1)
        .or_insert(1);
}

assert_eq!(letters[&'s'], 2);
assert_eq!(letters[&'t'], 3);
assert_eq!(letters[&'u'], 1);
assert_eq!(letters. get(&'y'), None);

use std::collections::HashMap;

let mut map: HashMap<&str, u32> = HashMap::new();

map. entry("poneyland").or_insert(3);
assert_eq!(map["poneyland"], 3);

*map. entry("poneyland").or_insert(10) *= 2;
assert_eq!(map["poneyland"], 6);
```

### 9. 合并 `HashMap`：`extend`
`extend` 方法允许将另一个 `HashMap` 的所有键值对合并到当前的 `HashMap` 中。

```rust
use std::collections::HashMap;

fn main() {
    let mut map1 = HashMap::new();
    map1.insert("苹果", 3);
    
    let mut map2 = HashMap::new();
    map2.insert("香蕉", 2);

    // 合并两个 HashMap
    map1.extend(map2);

    println!("{:?}", map1); // 输出: {"苹果": 3, "香蕉": 2}
}

use std::collections::HashMap;

fn main() {
    let mut map1 = HashMap::new();
    map1.insert("苹果", 3);

    let a = [("banana", 2)];

    map1.extend(a);
    println!("{:?}", map1); // 输出: {"苹果": 3, "香蕉": 2}
}
```

### 10. 获取键的数量：`len`
`len` 方法返回 `HashMap` 中键值对的数量。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("苹果", 3);
    map.insert("香蕉", 2);

    println!("键值对的数量是: {}", map.len()); // 输出: 2
}
```

### 11. 清空 `HashMap`：`clear`
`clear` 方法可以清空 `HashMap`，删除所有的键值对。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("苹果", 3);
    map.insert("香蕉", 2);

    map.clear();
    println!("清空后的 HashMap: {:?}", map); // 输出: {}
}
```

### 总结
`HashMap` 是一个功能强大的数据结构，提供了常用的增删改查、迭代、条件插入等功能，适用于需要键值对存储和快速查找的场景。在 Rust 中，`HashMap` 的使用很灵活，支持多种操作，并且通过借用检查和所有权管理来保证内存安全。
