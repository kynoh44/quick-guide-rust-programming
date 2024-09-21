#[derive(Debug)]
enum WEEK {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

fn main() {
    let today: WEEK = WEEK::Sunday;
    //println!("{}", today); // compile error!!!
    println!("{:?}", today);
    //let tomorrow: WEEK = 22; // compile error!!!

    match today {
        WEEK::Sunday => println!("Sleep"),
        //_ => println!("Study"), // try again after removing this line
    }
}
