extern crate rand;
use rand::{thread_rng, Rng};

pub fn generate_secret() -> u32 {
    let mut rng = thread_rng();
    //let secret: u32 = rand::random::<u32>();
    let secret: u32 = rng.gen_range(0, 10);
    return secret;
}

pub fn compare(guess: &u32, secret: &u32) -> bool {
    if guess == secret { return true; }
    else { return false; }
    /*match secret.cmp(guess) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!")
    }*/
}