use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("in.txt").expect("in.txt does not exist");
    let mut rules = fs::read_to_string("rules.txt").expect("rules.txt does not exist");
    let exist = kill_qq(contents, &mut rules);
    fs::write("rules.txt", rules.as_str()).expect("write rules.txt failed");
    fs::write("exist.txt", exist.as_str()).expect("write exist.txt failed");

    let mut undo = String::new();
    let mut cnt = 0;
    loop {
        undo.push_str(format!("undo {}\n", cnt).as_str());
        cnt += 10;
        if cnt > 2000 {
            fs::write("undo.txt", undo.as_str()).expect("write undo.txt failed");
            break;
        }
    }

    let not = fs::read_to_string("not.txt").expect("not.txt does not exist");
    let s = undo_rules(not, &mut rules);
    fs::write("exclude.txt", s.as_str()).expect("exclude.txt failed to write");
}

fn kill_qq(contents: String, rules: &mut String) -> String {
    let re = Regex::new(
        r"((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})(\.((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})){3}",
    )
    .unwrap();
    let mut exist = String::new();
    for line in contents.lines() {
        for cap in re.captures_iter(line) {
            if !&cap[1].eq("10") {
                if rules.find(&cap[0]).is_none() {
                    rules.push_str(format!("network host address {}\n", &cap[0]).as_str());
                    println!("RET: {}", &cap[0]);
                } else {
                    exist.push_str(format!("undo network host address {}\n", &cap[0]).as_str());
                }
            }
        }
    }

    println!("{}", rules);
    exist
}

fn undo_rules(not: String, rules: &mut String) -> String {
    let mut undo = String::new();
    let re = Regex::new(
        r"((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})(\.((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})){3}",
    )
    .unwrap();
    for line in rules.lines() {
        let ip = re.captures(line).unwrap();
        let mut exclude = false;
        for ex in not.lines() {
            if ip[0].starts_with(ex) {
                exclude = true;
                break;
            }
        }
        if exclude {
            undo.push_str(format!("undo network host address {}\n", &ip[0]).as_str());
        }
    }

    println!("{}", undo);
    undo
}
