fn main() {
    let some_option = Some("Hello");

    if let Some(unwrapped_value) = some_option {
        println!("{}", unwrapped_value); // 输出: Hello
    } else {
        println!("some_option is None");
    }
}
