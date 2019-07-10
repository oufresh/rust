use std::io;
mod guess_game;

pub fn guess_game() {
    println!("----------------------------");
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error reading line!!");

    let guess_num: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed {}", guess_num);

    let secret = guess_game::generate_secret();
    let win = guess_game::compare(&guess_num, &secret);

    if win {
        println!("YOUWIN :) :)");
    } else {
        println!("YOU LOOSE :'(, the secret is {}", secret);
    }
}
