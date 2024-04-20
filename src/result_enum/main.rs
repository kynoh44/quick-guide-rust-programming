fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        return Err(String::from("denominator cannot be zero"));
    }
    Ok(numerator / denominator)
}

fn say_hello() -> Result<String, String> {
    Ok(String::from("hello"))
}

fn main() {
    let result = say_hello();
    match result {
        Ok(message) => println!("Say: {}", message),
        Err(message) => println!("Error: {}", message),
    }
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("Error: {}", message),
    }
}
