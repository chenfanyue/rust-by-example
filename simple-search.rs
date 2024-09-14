fn find_value(vec: &Vec<i32>, target: i32) -> Option<usize> {
    for (index, &value) in vec.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    match find_value(&numbers, 3) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Value not found"),
    }
}
