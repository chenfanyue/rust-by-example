fn main() {
    let mut values = vec![1, 2, 3, 4];
    values.push(5);

    for value in &values {
        println!("value = {}", value);
    }

    if values.len() > 5 {
        println!("List is longer than five items");
    }

    // Pattern matching
    match values.len() {
        0 => println!("Empty"),
        1 => println!("One value"),
        2..=10 => println!("Between two and ten values"),
        11 => println!("Eleven values"),
        _ => println!("Many values"),
    };

    // while loop with predicate and pattern matching using let
    while let Some(value) = values.pop() {
        println!("value = {value}"); // using curly braces to format a local variable
    }
}