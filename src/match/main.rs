fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} - FizzBuzz", i),
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            (_, _) => (),
        }
    }
}

fn main() {
    let age = 44;
    let gen = match age {
        0..=20 => "MZ",
        21..=50 => "X",
        51..=100 => "A",
        _ => "?",
    };

    for i in 1..=100 {
        let msg = match i {
            n if n % 15 == 0 => format!("{} - FizzBizz", n),
            n if n % 3 == 0 => format!("{} - Fizz", n),
            n if n % 5 == 0 => format!("{} - Buzz", n),
            _ => format!("{}", i),
        };
        println!("{}", msg);
    }
}
