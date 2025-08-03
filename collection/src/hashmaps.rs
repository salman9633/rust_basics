use std::collections::HashMap;
pub fn hash_maps() {
    /* common hashmaps operations
     - insert
     - get
     - remove
     - clear
    */

    /*

    hashmaps stores data in key value pair in rust , similar to object in rust
    need to import from library first
    */

    let mut users = HashMap::new();

    users.insert(String::from("salman"), 24);
    users.insert(String::from("faris"), 2);
    users.insert(String::from("leo"), 21);
    users.insert(String::from("jose"), 28);
    users.insert(String::from("thomas"), 12);
    users.insert(String::from("carius"), 15);

    let age_find = users.get("fars");

    // println!("age is {}", age_find);
    let (name, age) = hashmap_iteration(&users);
     println!("age Vec return {:?}", age);
    println!("name Vec return {:?}", name);
    match age_find {
        Some(age) => println!("age is {}", age),
        None => println!("no age found"),
    }
}

fn hashmap_iteration(users: &std::collections::HashMap<String, i32>)->(Vec<&String>, Vec<&i32>) {
    let mut name_vec = Vec::new();
    let mut age_vec = Vec::new();
    for (name, age) in users {
        name_vec.push(name);
        age_vec.push(age);
    }

    println!("user hash{:?}", users);
    println!("age Vec{:?}", age_vec);
    println!("name Vec{:?}", name_vec);

    return (name_vec, age_vec);
}
