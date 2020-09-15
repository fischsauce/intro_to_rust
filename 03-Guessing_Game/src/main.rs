use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\nGuess the number...\n");

    let secret_number = rand::thread_rng().gen_range(1,101);
    // let secret_number = rand::random::<u128>();
    // println!("The (not-so) secret number is {}", secret_number);

    loop {
        println!("Please to be inputting your guestimation:");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // ("Please type a number");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Wow, you got it!");
                break;
            }
        }
    }
}
