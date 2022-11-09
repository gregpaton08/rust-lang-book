/*
 * Chapter 2
 */

use std::io; // Use the io library from the standard library to read user input. Similar to C++
             // 'using std;'.
use rand::Rng;
use std::cmp::Ordering; // enum for comparing, values are Less, Greater, and Eqaul.

fn main() {
    println!("Guess the number");

    // secret_number is of type i32, the default numeric type in Rust.
    // However, since we are comparing it to guess below, and the type of guess is u32, the
    // compiler will infer that this is also be u32 and set it to that type.
    let secret_number = rand::thread_rng() // Create a random number generator that runs on this thread and that is seeded by the OS.
        .gen_range(1..=100); // Generate a random number in the range [1, 100].

    // Creates and infinite loop.
    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            // returns an io::Result.
            .read_line(&mut guess)
            // method of io::Result that is called if the value is Err. Causes program to crash and
            // print message. Without this line the program will compile with a warning since the
            // the return value is not being used and may contain an error.
            .expect("Failed to read line");

        // Rust allows us to shadow the previous type/value of guess with a new one.
        // Since this type is u32, and we later compare it to `guess`, Rust will infer that `guess`
        // should _also_ be of type u32. Woah!
        let guess: u32 = match guess
        .trim()  // remove any leading or trailing whitespace
        .parse() // parse the string to a different type, in this case u32
        {
            Ok(num) => num,
                        // The underscore is a catch-all to match any error value.
        Err(_) => {
            println!("Guess again, idiot!");
            continue;
        },
        };

        // think of {} as little crab pincers that hold a value in place.
        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
