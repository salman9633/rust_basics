fn main() {
    //by default, variables are immutable in rust
    let mut x = 5; // if we assign like let x=5 then you can reassign 6 to x next time
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
