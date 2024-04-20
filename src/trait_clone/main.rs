#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Clone for Book {
    fn clone(&self) -> Self {
        Book {
            title: self.title.clone(),
            author: self.author.clone(),
            published: self.published,
        }
    }
}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    let another = Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    };

    let mut book_clone = book.clone();
    println!("{:?}", book_clone);
    book_clone.clone_from(&another);
    println!("{:?}", book_clone);
}
