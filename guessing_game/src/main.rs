use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number!");
    loop {
        println!("Please Input Your Guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Please Type A Number!"); continue},
        };
        println!("You guessed: {}", guess);

        let random_number = rand::thread_rng().gen_range(1..=100);
        println!("The Number Is: {random_number}");

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
        }
    }
}
