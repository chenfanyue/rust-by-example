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

    match result1 {
        Ok(value) => println!("Result1: Success - {}", value),
        Err(e) => println!("Result1: Error - {}", e),
    }

    match result2 {
        Ok(value) => println!("Result2: Success - {}", value),
        Err(e) => println!("Result2: Error - {}", e),
    }
}
