fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter()
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .sum();
    println!("{sum}");
}

fn main() {
    let numbers = vec![1, 2, 3];

    let mut sum = 0;
    for &num in &numbers {
        let x = num * 2;
        if x > 5 {
            sum += x;
        }
    }

    let _ = sum;
}
