use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // * ! means it's macro
    println!("Guess the number!");

    // * 1..101 <=> 1..=100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    loop {
        // * ::new() - associated (static) function
        let mut guess = String::new();

        // * returns Stdin
        io::stdin()
            // * read_line takes input and APPENDs it to mutable string
            .read_line(&mut guess)
            // * read_line return io::Result, which has expect() method
            // * expect() crashes program and returns Err with message if error occurred,
            // or returns value that Ok is holding
            .expect("Failed to read line");

        // * trim() removes \n (Enter) from the end
        // * OR let guess = guess.trim().parse::<u32>().expect("Please type a number!");
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        //
        // parse() returns Result enum, so we match it for Ok and Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ means catchall value
            Err(_) => continue,
        };

        // * cmp() method compares guess to secret_number and return Ordering variant
        match guess.cmp(&secret_number) {
            // * Ordering is enum like Result, which has variants Less, Greater, Equal
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed: {}", guess);
                break;
            }
        }
    }
}
