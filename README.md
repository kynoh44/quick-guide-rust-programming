
# A tour of Rust for C programmer

How to run an example

Each example is added in Cargo.toml as below.
```
[[bin]]
name = "function_for"
path = "src/function_for/main.rs"
```

Cargo can build and run each example as below.
```
gurugio@AL01945427:~/my-rust-book$ cargo build --bin function_for
   Compiling my-rust-book v0.1.0 (/home/gurugio/my-rust-book)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
gurugio@AL01945427:~/my-rust-book$ cargo run --bin function_for
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/function_for`
Hello, function_for!
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
```
