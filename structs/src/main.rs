struct User {
    name: String,
    age: u8,
    is_active: bool,
}

fn main() {
    /*
    A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group

    simply what object in javascript is a struct
    */
    let age=23;
    let user =User{
        name: String::from("Salman"),
        age,
        is_active: true
    };

    println!("The name of the user is: {} is {} old", user.name,user.age);
}
