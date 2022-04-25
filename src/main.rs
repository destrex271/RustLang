use std::{io, cmp::Ordering};
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let sec_num = rand::thread_rng().gen_range(1..101);

    println!("Please enter input your guess: ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read");

    println!("You guessed: {}", guess);

    match guess.cmp(&sec_num) {
        Ordering::Less => println!("But you guessed too Small!"),
        Ordering::Greater => println!("But you guessed too Big!"),
        Ordering::Equal => println!("You Win!")
    }

}