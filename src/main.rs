/* Chapter 2
    Introduces Rust concepts: let, match, functions, external crates.
*/

use std::io;
use rand::Rng; //This is a trait.
use std::cmp::Ordering; //Ordering enum

fn main() {

    println! ("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println! ("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        //Returns a Result Value. Result is an enumeration.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Rust allows us to shadow the previous value of guess wth a new one.
        // Shadowing lets us reuse the guess variable rather the forcing us to create two unique variables
        // colon after guess tells Rust we'll annotate the variable's type
        // u32 is an unsigned 32 bit integer
        // parse converts string to a number. It returns a Result type and Result is an enum that
        // has the variants Ok and Err.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //The underscore is a catch-all value for all Err values
        };

        println!("You guessed: {guess}");


        /* Match expression to decide what to do next based on which variant of Ordering was
           returned from the call to cmp with teh values in guess and secret number.
           A match expression is made up of arms. An arm consists of a pattern to match against,
           and the code that should be run if the value given to match fits that arm's pattern.
         */
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
