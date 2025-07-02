use std::io;

fn main() {
    println!("Guess the Number!");

    println!("Please input yoru guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read lines!");

    println!("You guessed: {}", guess);
}
