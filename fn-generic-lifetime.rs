fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("hij");
        result = longest(&string1, &string2);
    }
    println!("longest is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
