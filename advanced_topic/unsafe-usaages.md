在展开 Rust 1.80.1 大纲中的 "Unsafe Rust" 部分时，我们将专注于如何使用不安全代码块以及其操作的具体细节。可以参考以下内容作为补充：

---

### Unsafe Rust

Rust 的核心设计之一是内存安全，但有些时候需要进行更底层的操作，这些操作可能会打破编译器的安全检查。这时候就需要使用 `unsafe` 代码块。Rust 通过 `unsafe` 提供了对这些操作的控制，允许开发者在某些特定情况下绕过安全性检查。

#### 1. Unsafe 代码块

在 Rust 中，`unsafe` 代码块是告诉编译器 “我知道这里有潜在的风险，请允许我执行不安全的操作”。通常，Rust 编译器会阻止不安全的内存操作，但在 `unsafe` 代码块中，你可以绕过这些限制。

**语法：**

```rust
unsafe {
    // 在这里执行不安全的操作
}
```

**注意事项：**
- `unsafe` 并不关闭借用检查器，也不会忽略 Rust 的所有安全性。它只是允许某些特定的操作，而这些操作默认情况下会被 Rust 拒绝。
- 即使在 `unsafe` 代码块中，开发者依旧需要确保操作是安全的，Rust 并不会自动验证这点。

#### 2. 不安全操作

在 Rust 中，只有五种操作被认为是不安全的，需要放在 `unsafe` 代码块中：

##### a. 解引用裸指针 (Dereferencing Raw Pointers)

裸指针 `*const T` 和 `*mut T` 是 Rust 中的一种指针类型，它们不同于 Rust 的引用类型（`&T` 和 `&mut T`），不会受到编译器的借用检查器的保护。解引用裸指针是危险的，因为它绕过了 Rust 的内存安全机制。

```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    *r2 += 10;
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

**注意：**
- 裸指针可以通过 `as` 转换从引用创建。
- 使用裸指针时，你需要手动确保它们指向有效的内存地址，否则可能会引发未定义行为。

##### b. 调用不安全的函数或方法 (Calling Unsafe Functions or Methods)

Rust 中的某些函数或方法是标记为不安全的，它们通常涉及底层操作，如直接操作内存或与系统交互。调用这些函数必须放在 `unsafe` 代码块中。

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

##### c. 访问或修改可变的静态变量 (Accessing or Modifying Mutable Static Variables)

静态变量在程序的整个生命周期中都存在，它们是全局变量。然而，Rust 的静态变量是不可变的，如果你想使用可变静态变量（`static mut`），你必须在 `unsafe` 代码块中进行操作。

```rust
static mut COUNTER: u32 = 0;

unsafe {
    COUNTER += 1;
    println!("COUNTER: {}", COUNTER);
}
```

**注意：**
- 使用 `static mut` 可能会导致数据竞争（data race），因此需要格外小心。

##### d. 实现不安全的 Trait (Implementing Unsafe Traits)

Rust 允许你定义和实现标记为 `unsafe` 的 trait。通常，这类 trait 涉及非常底层的操作，它们要求开发者确保实现的正确性。

```rust
unsafe trait UnsafeTrait {
    fn dangerous(&self);
}

struct MyStruct;

unsafe impl UnsafeTrait for MyStruct {
    fn dangerous(&self) {
        println!("dangerous method in unsafe trait");
    }
}

fn main() {
    let unit_struct = MyStruct;
    unit_struct.dangerous();
}
```

##### e. 访问联合体中的字段 (Accessing Fields of a Union)

联合体（`union`）是允许不同类型的数据共用相同的内存空间的结构体。在 Rust 中，访问联合体的字段是一个不安全的操作，必须放在 `unsafe` 代码块中。

```rust
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {
    let u = MyUnion { f1: 1 };

    unsafe {
        println!("{}", u.f1);
        println!("{}", u.f2); // 未定义行为
    }
}
```

---

**总结：**

虽然 `unsafe` 代码块允许你进行一些不受 Rust 借用检查器约束的操作，但它并不意味着可以忽视安全性。开发者在编写不安全代码时，仍然需要确保代码不会导致内存错误、数据竞争或未定义行为。`unsafe` 的设计初衷是为了在需要时提供对底层操作的精细控制，而不是滥用它来逃避安全性检查。
