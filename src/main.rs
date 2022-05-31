use std::fs;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("in.txt").expect("in.txt does not exist");
    let r = kill_qq(&contents);
    println!("{}, {}", contents.len(), r);
}

fn kill_qq(contents: &String) -> String {
    println!("{}!", contents.len());
    for line in contents.lines() {
        println!("LINE: {}", line);
        for piece in line.split("-") {
            println!("SPLIT: {}", piece);
        }
    }
    let ret = String::from("return");
    ret
}
