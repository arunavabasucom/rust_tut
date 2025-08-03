use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess a number!");
    loop{
        println!("Please guess a number");
        //initilize a var with a new string
        let mut guess  = String::new();
        
        // generate a serect number 
        let sn = rand::thread_rng().gen_range(1,101);
        println!("The secrect number is:{}",sn);
    
        //input from the user
        io::stdin().read_line(&mut guess).expect("Failed to load");
    
    
        //parse it into a number
        // let guess:u32 = guess.trim().parse().expect("Type a number");
        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue
        };
    
        // println!("You gussed {}",guess);
        match guess.cmp(&sn){
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Equal => {
                println!("{}","You Win!".green());
                break;

            }
            Ordering::Greater => println!("{}","Too Big!".red())
        }

    }
}
