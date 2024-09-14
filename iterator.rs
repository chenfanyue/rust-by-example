fn first_word(s: &str) -> &str {
    for (i, item) in s.chars().enumerate() {
        if item == ' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn main() {
    let w = first_word("good morning");
    println!("{w}");
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];  // `&s[..i]` 仍然是 `&str` 类型
        }
    }
    &s[..]  // `&s[..]` 也仍然是 `&str` 类型
}

