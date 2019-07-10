

//use rand::prelude::*;

//static mut secret: u32 = 0;

pub fn generate_secret() -> u32 {
    //let mut rng = rand::thread_rng();
    //println!("char: {}", rand::random::<u32>());
    let secret: u32 = rand::random::<u32>();
    return secret;
    //let mut rng = thread_rng();
    //let i = rng.

    // a uniformly distributed value between 0 and 1:
    //let x: f64 = rng.gen();

    // simulate rolling a die:
    //let roll = rng.gen_range(1, 7);
    //let secret_number = rng.next_u32();
    //gen_range(1, 101);
    //secret = secret_number;
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