use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("please input your guess!");
        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read line!");
    
        // let guess: u32 = guess.trim().parse().expect("not a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("your input number is {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
