use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Warner Bors' presents .. Guessing Game!");
    println!("You must guess the generated number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed less"),
            Ordering::Greater => println!("You guessed more"),
            Ordering::Equal => {
                println!("Yay, you guessed right!");
                break;
            }
        }
    }

}
