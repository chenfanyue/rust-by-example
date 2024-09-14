fn main() {
    let string1 = String::from("abcd");
    let string2 = "hij";
    let result = longest(string1.as_str(), string2);

    println!("longest is {result}");
}

fn longest<'a>(_x: &'a str, _y: &'a str) -> &'a str {
    "spain"
}
