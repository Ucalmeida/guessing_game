use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("O número secreto é: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");
    
        let guess: u32 = match 
        guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Grande demais!"),
            Ordering::Equal => {
                println!("Voce venceu!");
                break;
            }
        }
    
        println!("You guessed: {guess}");
    }
}