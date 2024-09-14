#[derive(PartialEq)]
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // 变量匹配 Foo::Bar
    if Foo::Bar == a {
        println!("a is foobar");
    }
}


/* ---------------- */
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // 变量匹配 Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}
