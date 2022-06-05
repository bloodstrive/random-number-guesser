use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //prints to console
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop{
        println!("Please input your guess.");
        //creates mutable guess :: String
        let mut guess = String::new();
           //gets from std io the .readLine func and passes the ref of variable mut guess
        io::stdin().read_line(&mut guess)
            //error handling readLine contains 2 possible values OK sends result to var or Err we use
            //expect to handle that error
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(e) => {
                println!("{}",e);
                continue;
            }
        };
        //prints the result of mut guess
        println!("you guessed: {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
