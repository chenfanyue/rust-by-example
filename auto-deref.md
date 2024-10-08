在 Rust 中，数组类型和切片类型共享很多行为，尤其是在数组上调用某些方法时，Rust 会自动将数组转换为切片。这种机制是通过 **自动解引用（auto-deref）** 和 **Deref trait** 实现的，具体细节如下：

### 1. **数组自动转换为切片**

在 Rust 中，数组 `[T; N]` 和切片 `&[T]` 是不同的类型，但是数组可以自动转换为切片。当你调用数组上定义在切片上的方法时，编译器会隐式地将数组引用 `&[T; N]` 转换为切片 `&[T]`。

在你的例子中：

```rust
let mut arr = [1, 2, 3, 4]; // arr 是一个长度为 4 的数组
arr.swap(1, 2); // swap 是定义在切片 &[T] 上的方法
println!("{:?}", arr); // 输出：[1, 3, 2, 4]
```

这里的 `swap` 方法是定义在切片（`&[T]`）上的，而不是直接定义在数组（`[T; N]`）上。Rust 编译器会自动将数组 `arr` 转换为切片 `&mut [T]`，因此你可以直接对数组调用这个方法。这个自动转换过程由 Rust 的类型系统处理，称为 **自动引用转换**。

#### 举例：
- **`[T; N]`**：固定大小数组，存储在栈上。
- **`&[T]`**：切片，表示对数组中一段连续内存的引用。

当你调用 `swap` 时，Rust 自动将 `&mut [T; N]` 转换为 `&mut [T]`，并应用到切片上。

### 2. **自动解引用与 `Deref`**

Rust 中的数组类型实现了 `Deref` trait，允许数组通过 `&` 和 `&mut` 自动解引用为切片。这种自动解引用机制使得你可以在数组上调用很多切片的方法，而不需要手动将数组转换为切片。

`swap` 函数的定义如下：

```rust
fn swap(&mut self, a: usize, b: usize);
```

它被实现为 `&mut [T]` 的方法，但由于自动解引用的存在，你可以直接在数组上调用。

### 3. **方法调用背后的机制**

Rust 的数组类型 `[T; N]` 实际上**实现了一些切片的行为**。例如，你可以使用 `iter()`、`len()`、`swap()` 这些方法，尽管它们是定义在切片上的。这是通过 Rust 的自动转换和类型推导机制实现的。当你调用方法时，Rust 会尝试将数组引用自动转换为切片引用，使得切片上的方法也能作用于数组。

```rust
let arr: [i32; 4] = [1, 2, 3, 4];
// arr.len() 是有效的，因为 Rust 将数组隐式地转换为切片
println!("{}", arr.len()); // 输出 4
```

### 总结

在 Rust 中，数组 `[T; N]` 和切片 `&[T]` 是紧密相关的类型。Rust 的类型系统提供了自动解引用和切片转换机制，使你可以在数组上直接调用切片的方法。`arr.swap(1, 2)` 就是通过这个自动转换机制起作用的，因此数组能够调用切片的方法而不需要手动转换。

