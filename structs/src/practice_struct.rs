struct Book {
    title: String,
    author: String,
    page: u32,
}

pub fn create_book(title: String, author: String, page: u32) {
    let book = Book {
        title,
        author,
        page,
    };

    println!("{}",book.author);
    println!("{}",book.title);
    println!("{}",book.page);
}
