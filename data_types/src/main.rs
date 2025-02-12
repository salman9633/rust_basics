fn main() {
    /*
    Rust have to types of data types

    Scalar & Compound

    Scalar:
    Integers: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
    Floats: f32, f64
    Booleans: bool
    Characters: char

    Compound:
    Tuple
    Array
    */

    //Tuple
    let tup: (i32, f64, bool) = (500, 6.3, true);
    println!("Tuple: {:?}", tup);

    let newTup = (100, 3.14, "Hello");
    let (i, f, s) = newTup; //detructuring
    println!("Tuple: {s}");
    //We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access
    //indexing start from 0
    let first_tup = newTup.0;
    println!("First element of Tuple: {first_tup}");
}
