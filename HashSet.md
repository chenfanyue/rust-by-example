`HashSet` 是 Rust 标准库中的一种集合类型，用于存储不重复的值。`HashSet` 基于哈希表实现，操作时间复杂度为 O(1)，非常适合存储和快速查询唯一元素。我们将详细介绍 `HashSet` 的主要方法和使用示例。

### 1. 创建 `HashSet`

使用 `HashSet::new()` 创建一个空的集合：

```rust
use std::collections::HashSet;

fn main() {
    let mut set: HashSet<i32> = HashSet::new();
    println!("{:?}", set); // 输出：{}
}

use std::collections::HashSet;

fn main() {
    let set1 = HashSet::from([1, 2, 3, 4]);
    let set2: HashSet<_> = [1, 2, 3, 4].into();
    assert_eq!(set1, set2);
}
```

### 2. `insert`

`insert` 方法用于向 `HashSet` 中插入一个元素。如果元素已经存在，则不会插入，且返回 `false`。

```rust
fn main() {
    let mut set = HashSet::new();

    assert_eq!(set.insert(2), true);
    assert_eq!(set.insert(2), false);
    assert_eq!(set.len(), 1);

    println!("{:?}", set);
}
```

### 3. `contains`

`contains` 方法用于检查集合中是否包含某个元素。

```rust
fn main() {
    let mut set = HashSet::new();
    set.insert(1);
    
    if set.contains(&1) {
        println!("集合包含 1");
    } else {
        println!("集合不包含 1");
    }
}
```

### 4. `remove`

`remove` 方法用于从集合中删除某个元素。如果成功删除，返回 `true`，否则返回 `false`。

```rust
fn main() {
    let mut set = HashSet::new();
    set.insert(2);
    assert_eq!(set.remove(&2), true);
    assert_eq!(set.remove(&2), false);
}
```

### 5. `is_empty` 和 `len`

`is_empty` 方法返回集合是否为空，`len` 方法返回集合中元素的个数。

```rust
fn main() {
    let mut set = HashSet::new();
    println!("集合是否为空: {}", set.is_empty());  // 输出: true

    set.insert(1);
    println!("集合长度: {}", set.len());  // 输出: 1
}
```

### 6. `clear`

`clear` 方法用于清空集合。

```rust
fn main() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    
    println!("集合长度: {}", set.len());  // 输出: 2

    set.clear();
    println!("集合长度: {}", set.len());  // 输出: 0
}
```

### 7. 迭代 `HashSet`

可以使用迭代器遍历 `HashSet` 中的元素。

```rust
fn main() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    for item in &set {
        println!("{}", item);
    }
}
```

### 8. `union`, `intersection`, `difference`, 和 `symmetric_difference`

这些方法用于集合的操作：并集、交集、差集、对称差集。

- **`union`** 返回两个集合的并集。
- **`intersection`** 返回两个集合的交集。
- **`difference`** 返回集合 A 中有但集合 B 中没有的元素。
- **`symmetric_difference`** 返回两个集合的对称差集，即只出现在其中一个集合中的元素。

```rust
fn main() {
    let set1: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let set2: HashSet<_> = [3, 4, 5].iter().cloned().collect();

    // 并集
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("并集: {:?}", union);  // 输出: {1, 2, 3, 4, 5}

    // 交集
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("交集: {:?}", intersection);  // 输出: {3}

    // 差集
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("差集: {:?}", difference);  // 输出: {1, 2}

    // 对称差集
    let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
    println!("对称差集: {:?}", symmetric_difference);  // 输出: {1, 2, 4, 5}
}
```

### 9. `drain`

`drain` 方法将所有元素从集合中移出，并返回一个迭代器，可以在遍历的同时清空集合。

```rust
fn main() {
    let mut set: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    
    for item in set.drain() {
        println!("移除: {}", item);
    }
    println!("集合是否为空: {}", set.is_empty());  // 输出: true
}
```

### 10. `extend`

`extend` 方法可以用于从迭代器中插入多个元素。

```rust
fn main() {
    let mut set = HashSet::new();
    set.extend([1, 2, 3].iter().cloned());
    println!("{:?}", set);  // 输出: {1, 2, 3}
}
```

### 总结

`HashSet` 提供了丰富的方法来处理集合中的元素操作，支持高效的插入、删除、查找以及各种集合运算。在实际编程中，它非常适合用来去重和快速查询唯一值的场景。