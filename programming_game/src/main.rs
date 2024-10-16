use std::io;
fn main() {
    println!("Guess Your Number!");
    println!("Enter your Guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    println!("You guessed: {}", guess);
    
}
