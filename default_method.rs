trait Greeter {
    fn greet(&self);

    // 默认方法
    fn farewell(&self) {
        println!("Goodbye!");
    }
}

struct Person;

impl Greeter for Person {
    fn greet(&self) {
        println!("Hello!");
    }
}

fn main() {
    let person = Person;
    person.greet();      // 输出: Hello!
    person.farewell();   // 输出: Goodbye!
}
