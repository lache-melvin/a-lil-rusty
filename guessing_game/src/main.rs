use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\nWe're guessing stuff! (Well, numbers, to be precise...)");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("\nGive me your best guess (between 1 and 100 my friend)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("I failed to read what you said. Sorry 'bout that.");

        println!("Your guess was: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(numero) => numero,
            Err(_) => continue,
        };

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
