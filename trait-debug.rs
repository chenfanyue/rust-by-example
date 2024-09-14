#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Peter";
    let age = 35;
    let peter = Person{name: name, age: age};

    println!("{:#?}", peter);
}
