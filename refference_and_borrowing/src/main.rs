fn main() {
    /*
    A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
     */

    let s1 = String::from("hello");

    let s2 = &s1;
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}{}.", s1, len, s2);
    /*
    Here in this case we are calling or using s1 the even after it's reassigned to another variable.
    here actually we are ressigning with the '&' that means the refference of s1 is assigned to s2. same as we are passing the refference to the function
     */

    /* mutable refference */

    let mut a = String::from("Mutable Value A");
    // let b = &mut a;
    update_value(&mut a);

    println!("The value of a is: {}", a);//out put ==>The value of a is: Mutable Value A Updated Value

    // println!("The value of a is: {}", b);
}

fn calculate_length(s: &String) -> usize {
    //borrowing here
    return s.len();
}

fn update_value(s: &mut String) {//borrow muttably
    s.push_str(" Updated Value");
}