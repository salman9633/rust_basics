use std::io;

fn main() {
    pattern()
}

fn pattern() {
    /*

     *
     * *
     * * *
     * * * *
     * * * * *
    */
    println!("Tell How Many lines You want");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please Type A Number!");
            0
        }
    };

    let len = input+1;
    // let mut cur_line = 1;
    for i in 1..=len {
        for _ in 1..i {
            print!("* ");
        }
        println!();

        // if(i-1)
    }
}
