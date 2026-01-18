use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let ret_fn=error_fn();
    println!("Function Called ");
}

fn error_fn() {
    // let res = File::read_to_string("file.txt");
    //  match res {
    //     Ok(file) => file,
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         return;
    //     }
    // };

    read_a_file();
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

fn read_a_file(){
    let f = File::open("hello.txt");

    let f=match f {
        Ok(file)=>file,
        Err(error)=>match error.kind() {
            ErrorKind::NotFound=>match File::create("hello.txt") {
                Ok(fc)=>fc,
                Err(e)=>panic!("{:?}",e),
            },
            other_error=>{
                panic!("problem Opening {:?}",other_error)
            }
        }
        
    };
}