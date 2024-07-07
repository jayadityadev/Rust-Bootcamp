// use std::io;

// fn main() {

//     println!("Enter a number (string input): ");
//     let mut string_input = String::new();
//     io::stdin().read_line(&mut string_input).expect("msg");
//     println!("String input: {}", string_input);
//     let int_input: i64 = string_input.trim().parse().unwrap();
//     println!("After integer conversion: {}", int_input);
//     println!("Performing basic arithmetics: (input + 2): {}", int_input + 2);

// }

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

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