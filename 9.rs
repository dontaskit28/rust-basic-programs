use text_io::read;
use rand::Rng;
fn main() {
  let rand_number = rand::thread_rng().gen_range(1..=100);
  println!("Guess the number! Between 1 and 100");
  loop{
    let user_guess:i32 = read!();
    if user_guess == rand_number{
      println!("Congrats!.. You got it..");
      break;
    }
    else if user_guess < rand_number{
      println!("your guess is too low.. Try again..");
    }else{
      println!("your guess is too high.. Try again..");
    }
  }
}