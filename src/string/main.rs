fn moved_string(data: &str) {
    println!("{}", data);
}

fn main() {
    let hello = String::from("Hello, world!");
    let mut s = String::new();
    let s = "initial contents".to_string();
    let hello = String::from("안녕하세요");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let mut mutable_string = String::from("hello");

    moved_string(&mutable_string);
    println!("{}", mutable_string);

    let mut mutable_string = String::from("hello");
    println!("{}", mutable_string.chars().nth(0).unwrap());

    {
        let hello = "hell".to_string();
        let r1 = &hello;
        let mut r2 = &mut hello;
    }
}
