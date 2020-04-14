
use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {

    // generating random number
    let mut rng = rand::thread_rng();
    let x : u32 = rng.gen_range(1,100);

 
    loop {
        let mut guess = String::new();
        
        // references are immutable by default, need to add '& mut' or we wont be able to 
        // modify memory
       
        println!("Guess a Number!");
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to Read Line!");
        let guess : u32 = match  guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("The secret number is {}", x);
        match guess.cmp(&x) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!(" You Win, Congrats! ");
                break;
            }
        }

   }
}
