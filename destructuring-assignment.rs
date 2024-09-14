struct Example {
    a: bool,
    b: u64,
}

fn main() {
    let Example { a, b: _b } = Example { a: true, b: 10004 };
    println!("{a}");
}
