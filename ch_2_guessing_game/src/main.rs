use std::io;
use std::cmp::Ordering;
use rand::Rng; // from random range is used in our code
use colored::*; // Used for coloring the string in consoles output

fn main() {
    println!("Guess the Number!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    // This is the infinity loop if you want to stop in btw use quit in cmd
    loop {
        println!("Please input your guess:");
    
        // Variable declaration and assigning with empty new string
        let mut guess = String::new();
        println!("printing empty {guess}"); // this will print nothing
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
    
        /*
            Creating new variable with same name as above
                this is called shadowing which means creating same
                name variable but with different datatype
                here
         */
        // let guess: u32 = guess.trim().parse().expect("Please type a number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        // There are various ways to print variable ("{}, {}", guess, another) is also correct
        println!("Your guess is : {guess}");
    
        // this will be use to match which to compare guess value with secret number in which if it is less then it will print too smal;
        // else it will check if greater then it will print too big!! else if it is equal then it print You win and break the loop
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!!".green());
                break;
            }
        }
    }
}