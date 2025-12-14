fn main() {
    /*---------------- vector practice -------------------*/

    // Create an empty vector of i32 and push the numbers 1 to 5 into it.
    let mut vec1: Vec<i32> = Vec::new();

    for i in 1..=5 {
        vec1.push(i)
    }

    println!("{:?}", vec1);

    // Access the 3rd element of a vector:
    match vec1.get(2) {
        Some(index) => println!("The third Elem is {}", index),
        None => println!("Out of Index"),
    }

    // Note : let third = vec1[2] this is also we can use but it will return error while out of index time

    // Remove the last element from a vector and print it safely.
    vec1.remove(vec1.len() - 1);
    println!("{:?}", vec1);

    // Iterate over a vector and print each element without taking ownership.

    for x in &vec1 {
        // Use &v or v.iter() when you want to read values without taking ownership
        println!("x : {}", x);
    }

    // Double every element in a vector in-place.

    for x in &mut vec1 {
        *x *= 2
    }

    println!("{:?}", vec1);

    // Write a function that takes a vector and prints all elements.
    // First version: takes ownership
    // Second version: borrows the vector
    let vec2 = vec![String::from("Salman"), String::from("Faris")];
    take_vector_bor(&vec2);
    take_vector_own(vec2);

    // Write a function that returns the longest string from a vector of String
    let str_vec = vec![
        String::from("THRISSUR"),
        String::from("ERANAKULAM"),
        String::from("CALICUT"),
    ];
    let longest_string = find_longest_string(&str_vec);
    print!("The longest String is {}", longest_string);
}

fn take_vector_own(vector: Vec<String>) {
    for x in vector {
        println!("own X {}", x);
    }
}
fn take_vector_bor(vector: &Vec<String>) {
    for x in vector {
        println!("bor X {}", x);
    }
}

fn find_longest_string(vec: &Vec<String>) -> &String {
    let mut longest = &vec[0];
    for i in 1..vec.len() - 1 {
        if vec[i].len() > longest.len() {
            longest = &vec[i];
        }
    }

    return &longest;
}
