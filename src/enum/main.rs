enum MyEvent {
    NewFile(String),
    NewData { path: String, contents: String },
    Close,
}

fn events_handler(events: Vec<MyEvent>) {
    for ev in events.into_iter() {
        // ownership is moved!!
        match ev {
            MyEvent::NewFile(path) => println!("New file {} is created", path),
            MyEvent::NewData { path, contents } => {
                println!("New data \"{}\" in file {}", contents, path)
            }
            MyEvent::Close => println!("Event monitor is closed"),
        }
    }
}

fn main() {
    let create_file = MyEvent::NewFile("/root/conf.ini".to_string());
    let write_data = MyEvent::NewData {
        path: "/root/conf.init".to_string(),
        contents: "Hello!".to_string(),
    };
    let close_monitoring = MyEvent::Close;
    let mut events: Vec<MyEvent> = Vec::new();
    events.push(create_file);
    events.push(write_data);
    events.push(close_monitoring);
    events_handler(events);
}
