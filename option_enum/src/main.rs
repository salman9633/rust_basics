fn main() {
    /* 
    option enum in Rust is to handle the concept of nullablity.
    Unlike other lanhuages, Rust doesn't have null.

    just like error handling enum with Result keyword, Rust has Option enum.
    That looks similar like:
    pub enum Option<T> {
        None,
        Some(T),
    }
    */

    //if you ever have a function that returns the null, return an Option instead.
    let my_string=String::from("SALMAN");
    match find_a_character(my_string,'S'){
        Some(index)=>println!("Found at {}",index),
        None=>println!("Not Found"),
    }

}

fn find_a_character(word:String,chara:char)->Option<usize>{
    for (index,character) in word.chars().enumerate(){
        if character==chara{
            return Some(index);
        }
    }
    return None;
}
