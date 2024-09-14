// 以这个 enum 类型为例
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    // 创建变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 变量 a 匹配到了 Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
