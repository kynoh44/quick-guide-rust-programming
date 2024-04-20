fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(n) => println!("x is {}", n),
        None => println!("x is not present"),
    }

    match y {
        Some(n) => println!("y is {}", n),
        None => println!("y is not present"),
    }

    if let Some(n) = x {
        println!("x is {}", n);
    }

    if let Some(n) = y {
        println!("y is {}", n);
    } else {
        println!("y is not present");
    }

    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x is {}", x.unwrap());
    println!("y is {}", y.unwrap());

    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x is {}", x.unwrap_or(-1));
    println!("y is {}", y.unwrap_or_default());

    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    let item = y.expect("slice should not be empty");
}
