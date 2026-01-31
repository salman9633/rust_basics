/* Defining Trait */
trait Summary {
    fn summarize(&self) -> String;

    /* Default Trait */
    fn summarize_name(&self) -> String {
        //if we pass any thing then it takes thats return statment other wise this
        //It is okay not to use this one inside impl
        return String::from("His Name is Salman");
    }
}

/* Defining Struct */
struct User {
    name: String,
    age: u32,
}

/* Implementing the trait with struct */
impl Summary for User {
    fn summarize(&self) -> String {
        return format!("The Name is {} of {} years of old", self.name, self.age);
    }
}

/* Trait as params */

fn notify_user(u: impl Summary) {
    println!("{}", u.summarize_name());
}

fn main() {
    /*
    In Rust, traits are a core feature that define shared behavior.
    If structs define data, then traits define what that data can do.

    Like interface in TS & abstract in java
    */

    let user = User {
        name: String::from("SALMAN FARIS"),
        age: 24,
    };

    /* using that trait function to get what we want */
    println!("{}", user.summarize());
    println!("{}", user.summarize_name());
    notify_user(user); //we cant pass anything other than user here as summary trait implemented with User struct only we can implment Summary trait with other structs also
}

trait Printable {
    fn print_item(&self);
}

struct Document {
    title: String,
}

impl Printable for Document  {
    fn print_item(&self) {
        println!("Hello")
    }
}
