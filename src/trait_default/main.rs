#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let book = Book::default();
    let mut book_clone = book.clone();
    println!("{:?}", book_clone);
}
