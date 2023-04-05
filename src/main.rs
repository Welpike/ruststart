use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
  println!("Just price");

  let secret_number = rand::thread_rng().gen_range(1..101);

  loop{
    println!("Please enter a number");
    
    let mut user_input = String::new();
    
    io::stdin()
      .read_line(&mut user_input)
      .expect("Error during the recuperation of the input.");
  
    let user_input: u32 = match user_input.trim().parse(){
      Ok(number) => number,
      Err(_) => continue,
    };
    //user_input.trim().parse().expect("You didn't enter a number.");
  
    println!("Your input : {user_input}");
  
    match user_input.cmp(&secret_number) {
      Ordering::Less => println!("Greater"),
      Ordering::Greater => println!("Less"),
      Ordering::Equal => {
        println!("You win");
        break;
      },
    }
  }
}