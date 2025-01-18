use clap::{Arg, Command};

/// HOW TO RUN
/// It adds crate_clap to the [[test]] section in Cargo.toml with harness = false option.
/// Therefore, you can run the main function with the --test option.
///
/// $ cargo test --test crate_clap -- --userid 1 --productid asdfasdf
/// Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
/// Running tests/crate_clap.rs (target/debug/deps/crate_clap-2038d8ca9fcd5e3a)
/// Hello, world!
/// User ID: 1
///

fn main() {
    println!("Hello, world!");

    let command = Command::new("serial")
        .version("0.1.0")
        .about("Serial number generator")
        .arg(
            Arg::new("userid")
                .long("userid")
                .help("Sets the user ID")
                .required(false),
        )
        .arg(
            Arg::new("productid")
                .long("productid")
                .help("Sets the product ID")
                .required(false),
        );

    let command = command.arg(
        Arg::new("expiredate")
            .long("expiredate")
            .help("Sets the expire date"),
    );

    let matches = command.get_matches();

    // add expiredate
    if let Some(userid) = matches.get_one::<String>("userid") {
        println!("User ID: {}", userid);
    }
    if let Some(expiredate) = matches.get_one::<String>("expiredate") {
        println!("Expire date: {}", expiredate);
    }
}
