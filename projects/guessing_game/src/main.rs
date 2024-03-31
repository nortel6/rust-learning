use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 1..=100 is a kind of range expression
    // 1 to 100 inclusively
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // mut = mutable
        // Default is immutable
        // :: is associated function
        let mut guess = String::new();
    
        // If you hadn't import std::io
        // std::io::stdin()
        io::stdin()
            // &mut guess makes the reference to guess mutable
            .read_line(&mut guess)
            // expect is the error handler, optional but will warns the user
            .expect("Failed to read line");
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // match the Result type with Enum values (Ok, Err)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is catchall, means to match all sorts of Err
            Err(_) => continue,
        };
        // Python's fstring ftw
        println!("You guessed: {guess}");
    
        // Just for demonstration for the print insertion    
        // let x = 5;
        // let y = 10;
        // println!("x = {x} and y + 2 = {}", y + 2);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }    
    }
}
