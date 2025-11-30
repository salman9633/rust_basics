pub struct Book {
    pub title: String,
    pub author: String,
    pub page: u32,
}

struct Rectangle {
    width: u32,
    hieght: u32,
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

    let rect = Rectangle {
        width: 30,
        hieght: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        hieght: 20,
    };

    let rect2 = Rectangle {
        width: 80,
        hieght: 90,
    };

    println!("can rect hold rect1 {}", rect.can_hold(&rect1));
    println!("can rect hold rect2 {}", rect.can_hold(&rect2));

    let area = rect.area();
    let perimeter = rect.perimeter();
    println!("AREA {}, PERIMETE{}", area, perimeter);
}

/* method using struct */

impl Rectangle {
    fn area(&self) -> u32 {
        return self.hieght * self.width;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.hieght + self.width);
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.hieght > other.hieght;
    }
}
