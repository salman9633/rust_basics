use std::{fmt::Error, fs};
fn main() {
    let ret_fn=error_fn();
    println!("Function Called ");
}

fn error_fn() {
    let res = fs::read_to_string("file.txt");
     match res {
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}

/* 
In rust error handling work with an Enum called Result
It has two variants
Either Ok or Err
Ok is for success and Err is for failure
so we can use match to handle the response
the Result type is generic
which will looks similar like

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/