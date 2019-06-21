use std::io;

fn guess_game() {
    println!("----------------------------")
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error reading line!!");

    println!("You guessed {}", guess);
}