use std::fs;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("in.txt").expect("in.txt does not exist");
    let r = kill_qq(&contents);
    println!("{}, {}", contents.len(), r);
}

fn kill_qq(contents: &String) -> String {
    println!("{}!", contents.len());
    let re = Regex::new(r"((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})(\.((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})){3}").unwrap();
    for line in contents.lines() {
        println!("LINE: {}", line);
        for cap in re.captures_iter(line) {
            println!("RET: {}", &cap[0]);
        }
    }

    let ret = String::from("return");
    ret
}
