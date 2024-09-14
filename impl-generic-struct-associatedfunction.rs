struct Struct<T> {
    element: T,
}

impl<T> Struct<T> {
    fn new(e: T) -> Self {
        Struct { element: e }
    }
    fn other<O: std::fmt::Display>(v: O) {
        println!("{}", v);
    }
}

fn main() {
    let s: Struct<i32> = Struct::new(10);
    println!("{}", s.element);
    Struct::<f32>::other("associated function");
}
