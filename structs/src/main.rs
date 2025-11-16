mod practice_struct;

use practice_struct::create_book;
use practice_struct::Book;

struct User {
    name: String,
    age: u8,
    is_active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area_of_rectangle(&self) -> u32 {
        //this self variable have a access to the struct, just like we are using this keyword
        self.width * self.height
    }
}
fn main() {
    /*
    A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group

    simply what object in javascript is a struct
    */
    let age = 23;
    let user = User {
        name: String::from("Salman"),
        age,
        is_active: true,
    };

    println!("The name of the user is: {} is {} old", user.name, user.age);

    /*
    You can also implement structs, which means you can attach functions to instaces of the struct.
    very similar to classes in TS
    */

    let rect = Rectangle {
        width: 50,
        height: 20,
    };
    println!("The Area Of The Rectangle Is: {}", rect.area_of_rectangle());

    let title = String::from("Fire & blood");
    let author = String::from("George RR Martin");
    create_book(
        title,
        &author,
        250,
    );

    create_book(String::from("Dance Of Dragons"), &author, 30);

    let book1=Book::new("TALE OF SOUTH","ROBERT",30);

    println!("{}",book1.title);
    println!("{}",book1.author);
    println!("{}",book1.page);
    
}
