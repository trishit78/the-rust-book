use std::io; 
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game");
    
    println!("Guess the random number");
    let secret = rand::thread_rng().gen_range(1..=100);

    loop{

        
        println!("Enter a number");
    
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read the line");

let guess : u32 = match guess.trim().parse(){
    Ok (num) => num,
    Err(_)=>continue,
};
println!("Your guess is {guess}");
//println!("The secret number is {secret}");

match guess.cmp(&secret){
    Ordering::Less => println!("Too small"),
    Ordering::Greater => println!("Too Big"),
    Ordering::Equal =>{
        println!("You win");
        break;
    }
}

}
}
