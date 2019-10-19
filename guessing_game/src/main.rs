use std::io; // Use the io library from the standard library to read user input. Similar to C++
             // 'using std;'.
use rand::Rng;
use std::cmp::Ordering; // enum for comparing, values are Less, Greater, and Eqaul.

fn main() {
    println!("Guess the number");

    // secret_number is of type i32, the default numeric type in Rust.
    // However, since we are comparing it to guess below, and the type of guess is u32, the
    // compiler will infer that this is also be u32 and set it to that type.
    let secret_number = rand::thread_rng().gen_range(1, 101);

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

        // Rust allows you to shadow the previous value of guess with a new one.
        // The trim method removes leading and trailing whitespace.
        // The parse method will parse to the specified type, in this case u32.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // The underscore is a catch-all to match any error value.
            Err(_) => continue,
        };

        // Print a formatted string. The {} is a placeholder for a variable value, provided after
        // the string.
        println!("You guessed: {}", guess);

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
