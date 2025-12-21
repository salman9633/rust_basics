use std::collections::HashMap;

fn main() {
    //word counter
    let text = "YOU WILL NEVER WALK ALONE HERE NEVER EVER HERE ANYMORE";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    //What happens if you insert a value for an existing key?

    let mut new_map = HashMap::new();

    new_map.insert("k", 10);
    new_map.insert("l", 6);
    new_map.insert("k", 17); //will update the k's value from 10 to 17

    println!("{:?}", new_map);

    // Write code that increments all values by 1 while iterating.

    let mut hash_map: HashMap<String, i32> = HashMap::new();
    hash_map.insert(String::from("ROGER"), 25);
    hash_map.insert(String::from("MARTIN"), 29);
    hash_map.insert(String::from("ZUBIMENDI"), 5);
    hash_map.insert(String::from("TELLO"), 85);
    hash_map.insert(String::from("SYNDER"), 75);

    for val in hash_map.values_mut() {
        *val += 1;
    }
    println!("Incremented Hash Map {:?}", hash_map);

    // Conver a vector in to HashMap

    let mut vec: Vec<&str> = Vec::new();
    vec.push("My");
    vec.push("Name");
    vec.push("Is");
    vec.push("Thomas");
    vec.push("Shelby");

    let mut vec_map = HashMap::new();
    let mut index = 0;
    for i in vec {
        vec_map.insert(index, i);
        index += 1;
    }
    print!("Map {:?}", vec_map);
}
