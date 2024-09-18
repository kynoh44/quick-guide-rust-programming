fn make_greeting(name: &mut String) {
    name.push_str("씨 안녕하세요");
}

fn main() {
    let mut user = "페리스".to_string();
    make_greeting(&mut user);
    println!("{}", user);

    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.iter() {
        println!("{}", c);
    }
    println!("{:?}", user);

    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.clone().into_iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
