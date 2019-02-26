extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_ordering(secret: u32 , guess: u32) -> Ordering {
    guess.cmp(&secret)
}

// Returns number of guesses it took to get the answer
fn bin_search(secret: u32, lower: u32, upper: u32) -> u32 {
    let guess = (lower + upper) / 2;
    match get_ordering(secret, guess) {
        Ordering::Greater => 1 + bin_search(secret, lower, guess-1),
        Ordering::Less => 1 + bin_search(secret, guess+1, upper),
        Ordering::Equal => 1
    }
}

fn main() {

    let start = 1;
    let end = 10000;
    println!("Welcome to the guessing game! Your goal is to guess a number between {} and {} \
    in as few tries as possible. See if you can beat the computer :)", start, end);

    let secret = rand::thread_rng().gen_range(start, end + 1);

    let computer_guesses = bin_search(secret, start, end);
    println!("The computer took {} guesses", computer_guesses);

    let mut user_guesses = 0;
    loop {
        let mut guess = String::new();
        user_guesses += 1;
        println!("Enter a number between {} and {}", start, end);

        io::stdin().read_line(&mut guess).expect("Invalid input");
        let guess : u32 = guess.trim().parse().expect("Input must be non-negative int");

        let order = get_ordering(secret, guess);
        match order {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => { println!("Got em"); break }
        }
    }

    print!("You took {} guesses.. ", user_guesses);

    match user_guesses.cmp(&computer_guesses) {
        Ordering::Less => println!("You won!"),
        Ordering::Greater => println!("You lost :("),
        Ordering::Equal => println!("Tied. Not bad")

    }
}
