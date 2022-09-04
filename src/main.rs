use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number b/w (1,100): ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    let guess: i32 = guess.trim().parse().expect("Please enter a valid number");
    println!("You guessed {}", guess);
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!(
                "Your guess was a little less; Actual number was: {}",
                secret_number
            );
        }
        Ordering::Greater => {
            println!(
                "Your guess was a little greater; Actual number was: {}",
                secret_number
            );
        }
        Ordering::Equal => {
            println!("Bingo! An exact match");
        }
    }
}
