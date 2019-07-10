use std::io;
mod guess;

fn run_guess_game() {
    println!("Hai scelto guess game!");
    guess::guess_game();
    println!("---------------------------------");
    println!("---------------------------------");
}

fn main() {
    println!("---------------------------------");
    println!("Hello we have some test with rust");
    println!("");

    let mut exit: bool = false;

    while !exit {
        println!("Choose an example to run");
        println!("1. guess example");
        println!("2. box memory example");
        println!("q. exit");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Try again");
        if guess.trim() == "q" {
            exit = true;
        } else {
            match guess.trim() {
                "1" => run_guess_game(),
                "2" => println!("premuto 2"),
                _ => println!("non conosco!"),
            }
        }
    }

    println!("------------------------------------");
}
