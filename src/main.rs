use std::fs;

fn kill_qq() {
    for number in 1..=5 {
        println!("{}!", number);
    }
}

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("in.txt").expect("in.txt does not exist");
    println!("{}", contents);
    kill_qq();
}
