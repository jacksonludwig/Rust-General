use std::cmp::Ordering;
use std::io;

use rand::Rng;

// similar to import, gives static methods and others
fn main() {
    println!("---Guess the number---");
    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input a guess (1-100)");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // takes in a mutable reference to put input into
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // continue goes directly to next loop iteration
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win.");
                break;
            }
        }
    }
}