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

    //Array
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    //every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let newArr:[i32;5] = [1, 2, 3, 4, 5];
    //in above the sqaure brackets tell the compiler that this is an array of 5 elements of type i32

    //You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square bracket
    let same_value_array = [3; 5];
    //the above will be same like [3, 3, 3, 3, 3]
    println!("Same Value Array: {:?}", same_value_array);

    //access an array element using indexing
    let a = same_value_array[0];
    println!("First element of Array: {a}");
     
     new_function();
}

fn new_function() {//Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words
    println!("This is a new function");
}
