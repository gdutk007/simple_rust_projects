
use std::io;
use rand::prelude::*;

fn main() {

    // generating random number
    let mut rng = rand::thread_rng();
    let x : u32 = rng.gen_range(1,100);

    let mut guess = String::new();
    // references are immutable by default, need to add '& mut' or we wont be able to 
    // modify memory
    println!("Guess a Number!");
    io::stdin().read_line(&mut guess)
        .expect("Failed to Read Line!");

   let mut number: u32 = guess.trim().parse().unwrap();

   let mut done = false;
    while !done {
        println!("You Guessed {} ", guess.trim());
        println!("The Secret Number is : {}", x);   
        if number == x {
            done  = true;
            println!("Correct! You guessed the secret Number!");
        }else{
            println!("Incorrect, Guess Another Number!");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)
                .expect("Crashed reading user Input!");
            number = guess.trim().parse().unwrap();
        }
    }
}
