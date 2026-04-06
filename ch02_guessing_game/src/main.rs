use std::io;
use std::cmp::Ordering; // An enum with variants: Less, Greater, Equal
use rand::Rng; // 'Rng' is a trait that defines methods for random number generators
use colored::*; 

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // 'let mut' is required because variables are immutable by default in Rust
        // String::new() creates a growable, UTF-8 encoded string on the heap
        let mut guess = String::new();


        // .read_line() returns a Result enum (Ok or Err)
        // .expect() is a crash-safe way to handle errors during learning
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // SHADOWING: We reuse the name 'guess' but change the type from String to u32
        // .trim() removes whitespace/newline (\n) from the enter key
        // We use a 'match' here to handle invalid (non-number) input without crashing
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, // If parsing works, return the number
            Err(_) => continue,
        };

        println!("You guessed:{}", guess);

        // MATCH EXPRESSION: The heart of Rust logic
        // This is like a type-safe 'switch' statement that must cover all possibilities
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal =>{
                println!("{}","You win!".green());
                break; // Exit the loop when the guess is correct
            },
        }
   }
    
}


// 1. use rand crate to generate a random number
// 2. use std::io to read input, trim()
// 3. convert a variable to another type using something like :u32
// 4. use std::cmp compare two variables
// 5. loop(continue+break)
// 6. match
// 7. Result type
// 8. use colored crate to print text in colors 