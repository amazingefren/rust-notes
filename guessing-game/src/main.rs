// use std::io;
/*
    std::io includes stdin (aka read input prompt)
    so if not using std::io, we could use std::io::stdin much like cpp being std::cout
*/

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");
    let winning_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Enter Your Guess!: ");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You Guessed: {}", guess);
        match guess.cmp(&winning_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}

fn main_old() {
    // This is a comment
    println!("Guess the number!");
    println!("Please input your guess.");
    /*
        let foo = 5; == immutable, cannot be changed
        let mut foo = 5; == mutable, can be changed
        String = UTF-8 endcoded text
        :: syntax indicates associated function of String type. aka type implementation
        String::new() == Typeof String, ::new == create new empty String
    */
    // String must be used because stdin inputs a string, must be converted to other types if needed
    // let mut guess = String::new(); // mutable new empty string type.
    // std::io::stdin()
    /*
        read_line() method takes users standard input and place into typeof string.
        (&mut guess) is where the read_line will be stored.
            - hence the mutable importance, otherwise 'guess' could not be reassigned
        '&' indicates that the argument is a reference, allows code to access
            - data without creating data copies.
    */
    // .read_line(&mut guess) // cin >> points to mutable guess
    /*
        expect() method is an error handler
    */
    // .expect("Failed to read line"); // error handler
    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Error Reading Input");

    let guess: u32 = guess.trim().parse().expect("Expected a Number!");

    println!("You guessed: {}", guess); // return values
    let random_number = rand::thread_rng().gen_range(1, 101); // generate random number using rand::Rng

    // let random_number: u32 = 5;
    println!("Random Number: {}", random_number);

    // let number_guess = guess.parse().unwrap_or(0); // this method of conversion does not have error handling and will default to 0 if uncovertable
    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => {
            println!("You Win!");
            /*
                print! is cout << without endl so \n must be given or unix may cover last line (EOF break);
                println! is essentially cout << params << endl; EOF endline given.
            */
            print!("Test");
            print!(" --- Test2");
            print!("\n")
        }
    }
}
