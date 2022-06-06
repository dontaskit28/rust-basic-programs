use std::io::Write;
use std::fs::OpenOptions;

fn main() {

    let mut line = String::new();
    println!("Enter a line: ");
    std::io::stdin().read_line(&mut line).unwrap();

    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open("input.txt")
        .expect("Unable to open file");
    f.write_all(line.as_bytes()).expect("Unable to write data");
}