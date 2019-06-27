extern crate rand;
use rand::Rng;

static mut secret: u32 = 0;

pub fn generate_secret() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.next_u32();
    //gen_range(1, 101);
    //secret = secret_number;
}

pub fn compare(guess: &u32) {
    /*match secret.cmp(guess) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!")
    }*/
}