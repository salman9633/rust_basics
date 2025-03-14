fn main() {
    /*
    Ownership is a set of rules that govern how a RUST pgm manages memory.
    */

    /*
    String data store in heap and in stack it will store the heap's pointer value with length and capacity
    */

    //CASE 1
    let s1: String = String::from("hello");
    let s2 = s1;
    println!("First String Here:{}", s1); //In this case it will give you Error because s1 is already moved to s2 and s1 is no longer valid

    /*
    process Explanation===>

    -IN the above case first hello is created in heap and pointer it's address to s1 in stack
    -then the s1 is moved to s2
    -Here now the s2 is pointing the address to the heap memory so s1 is no longer valid
    -We can't point toward the same address simultaneously
    -When ever a variable goes out of scope, which ever data it is owning it will also goes out of scope/dropped.
    -you can freely use the s1 variable before the that s1 is assigned to s2.

    */

    //CASE 2

    let my_str = String::from("Another Case");

    take_ownership(my_str.clone());
    println!("The value of my_str is: {}", my_str); //In this case it will give you Error

    /*
    Same error case will follw here also

    we can avoid the error by using clone method, what it will do is it will create another "Another Case" string in heap
    */ 
}

fn take_ownership(val: String) {
    println!("The value of my_str is: {}", val)
}
