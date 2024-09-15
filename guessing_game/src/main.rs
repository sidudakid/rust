use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main(){
    loop {
        println!("{}","[!] Guess the number: [!]".blue());
        println!("Please input the number: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to get input");
        let secret_number = rand::thread_rng().gen_range(1,101);
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("{}","Error Occured".red());
                continue},
        };

        println!("Your guess is : {}", guess);
        println!("The secret number is: {}", secret_number);        

    //comparing if the secret number and the user input 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Greater => println!("{}","Tooo big".red()),
            Ordering::Equal => {
                println!("{}","you win".green());
                break;
            }
        }
    }
}
