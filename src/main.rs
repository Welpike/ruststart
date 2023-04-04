use std::io;


fn main() {
  println!("Just price");
  println!("Please enter a number");

  let mut user_input = String::new();

  io::stdin()
    .read_line(&mut user_input)
    .expect("Error during the recuperation of the input.");

  println!("Your input : {user_input}");
}