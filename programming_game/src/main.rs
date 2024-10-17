use std::io;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100); // this line will generate a random number between 1 and 100
    println!("The secret number is: {}", secret_number);
    println!("Guess Your Number!");
    println!("Enter your Guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    
}
