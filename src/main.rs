use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("in.txt").expect("in.txt does not exist");
    let mut rules = fs::read_to_string("rules.txt").expect("rules.txt does not exist");
    kill_qq(contents, &mut rules);
    fs::write("rules.txt", rules.as_str());
}

fn kill_qq(contents: String, rules: &mut String) -> usize {
    println!("{}!", contents.len());
    let re = Regex::new(
        r"((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})(\.((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})){3}",
    )
    .unwrap();
    for line in contents.lines() {
        println!("LINE: {}", line);
        for cap in re.captures_iter(line) {
            if ! &cap[1].eq("10") && rules.find(&cap[0]).is_none() {
                rules.push_str(format!("network host address {}\n", &cap[0]).as_str());
                println!("RET: {}", &cap[0]);
            }
        }
    }

    println!("{}", rules);
    rules.len()
}
