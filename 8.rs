use std::io;

struct Number{
    num:i32,
}

pub trait Error{
    fn custom_error(&self);
}

impl Error for Number{
    fn custom_error(&self){
        if self.num < 0{
            println!("Number is negative.");
        }
    }
}
fn main(){

    // user input
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read");

    let number = Number{
        num : input.trim().parse::<i32>().unwrap()
    };

    number.custom_error();
}