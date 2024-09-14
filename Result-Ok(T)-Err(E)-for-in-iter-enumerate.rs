fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Division by zero error"))
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    let results = vec![result1, result2];

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("Result{}: Success - {}", i + 1, value),
            Err(e) => println!("Result{}: Error - {}", i + 1, e),
        }
    }
}
