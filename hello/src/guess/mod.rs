use std::io;
mod guess_game;

pub fn guess_game() {
    let secret = guess_game::generate_secret();
    println!("----------------------------");
    println!("Guess the number from 0 to 10!");
    let mut attempts: u32 = 5;
    while attempts > 0 {
        println!("");
        println!("You have {} attempts", attempts);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line!!");

        let guess_num: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input number");
                continue;
            }
        };

        println!("You guessed {}", guess);

        let win = guess_game::compare(&guess_num, &secret);

        match win {
            guess_game::GuessResult::Greater => println!("TOO SMALL GUESS :( :("),
            guess_game::GuessResult::Equal => {
                println!("YOUWIN :) :)");
                break;
            }
            guess_game::GuessResult::Less => println!("TOO BIG GUESS :( :( ")
        }

        attempts = attempts - 1;
    }
}