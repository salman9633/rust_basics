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

    let age_find = users.get("fars");

    // println!("age is {}", age_find);
    match age_find {
        Some(age)=> println!("age is {}", age),
        None=>println!("no age found")
    }
   
}
