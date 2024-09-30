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

fn fizzbuzz_3(max: i32) {
    let ret = (1..=max)
        .into_iter()
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => format!("{} - FizzBuzz\n", i),
            (0, _) => format!("{} - Fizz\n", i),
            (_, 0) => format!("{} - Buzz\n", i),
            (_, _) => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);
}

fn main() {
    fizzbuzz_2(37);
    fizzbuzz_3(41);


    let mut maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.as_ref().map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));
    println!("{:?}", maybe_some_string);

    maybe_some_string
        .as_deref_mut()
        .map(|x| x.make_ascii_uppercase());
    println!("{:?}", maybe_some_string);
}
