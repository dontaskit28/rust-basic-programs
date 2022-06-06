use std::env;

fn main(){

    // command line arguments
    let args:Vec<String> = env::args().collect();
    if args.len() != 3{
        println!("Incorrect number of arguments..");
        return;
    }

    // converting string to i32
    let num1:i32 = args[1].parse().unwrap();
    let num2:i32 = args[2].parse().unwrap();
    
    println!("sum of {} and {} is {}",num1,num2,num1+num2);
}