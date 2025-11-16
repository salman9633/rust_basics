pub struct Book {
    pub title: String,
    pub author: String,
    pub page: u32,
}

impl Book {
    pub fn new(title: &str, author: &str, page: u32) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            page,
        }
    }
}

pub fn create_book(title: String, author: &String, page: u32) {
    let book = Book {
        title,
        author: author.clone(),
        page,
    };

    // Book::new();

    println!("====================");
    println!("{}", book.author);
    println!("{}", book.title);
    println!("{}", book.page);
    println!("====================");
}
