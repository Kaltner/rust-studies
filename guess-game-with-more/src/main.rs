use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let num_to_be_guessed = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number from 0 to 100");

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&num_to_be_guessed) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
